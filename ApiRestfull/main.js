import express from 'express';
import mongoose from 'mongoose';
import helmet from 'helmet';
import morgan from 'morgan';
import rateLimit from 'express-rate-limit';
import cors from 'cors';
import dotenv from 'dotenv';
import search from './configAD.js';
import db from './dbinfos.js';
import buscarSetorLDAP from './configAD.js';
const { Infos, User, Depart } = db;

dotenv.config();

const app = express();
const port = process.env.PORT;
const password = process.env.API_PASSWORD;
const MONGODB_URI = process.env.MONGODB_URI;
const SERVER_ADDRESS = process.env.SERVER_ADDRESS;

app.use(helmet());
app.use(morgan('combined'));
app.use(cors());
app.use(express.json({ limit: '1mb' }));

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  standardHeaders: true,
  legacyHeaders: false,
});
app.use(limiter);


const connectWithRetry = async () => {
  try {
    await mongoose.connect(MONGODB_URI);
    console.log('Conectado ao MongoDB com sucesso');
  } catch (err) {
    console.error('Erro ao conectar ao MongoDB:', err);
    console.log('Tentando reconectar em 5 segundos...');
    setTimeout(connectWithRetry, 5000);
  }
};

connectWithRetry();

function verifyRequest(req, res, next) {
  try {
    const senha = req.body.apiauth || req.headers['apiauth'] || req.query.apiauth;

    if (!senha || senha !== process.env.API_PASSWORD) {
      return res.status(401).json({ message: 'Acesso negado: autenticação requerida' });
    }
    next();
  } catch (error) {
    console.error('Erro na verificação de autenticação:', error);
    return res.status(500).json({ message: 'Erro interno no servidor' });
  }
}

const processarListaProgramas = (programs) => {
  let listaAplicativos = [];
  
  if (!programs) return listaAplicativos;
  
  if (Array.isArray(programs)) {
    listaAplicativos = programs.map(p => 
      typeof p === 'string' ? p : (p.nome || String(p))
    );
  } 
  else if (typeof programs === 'string' && programs.trim().startsWith('[')) {
    try {
      const parsedPrograms = JSON.parse(programs);
      listaAplicativos = parsedPrograms.map(p => p.nome || String(p));
    } catch (e) {
      console.log('Erro ao processar JSON de programas:', e.message);
      listaAplicativos = [String(programs)];
    }
  }
  else {
    listaAplicativos = [String(programs)];
  }
  
  return listaAplicativos;
};

app.post('/checkdepartament', verifyRequest, async (req, res) => {
  const data = Array.isArray(req.body) ? req.body : [req.body];
  const responses = [];

  for (const item of data) {
    try {
      const { username, apiauth } = item;

      if (!apiauth || !username) {
        responses.push({ status: 400, message: 'Autenticação obrigatória' });
        continue;
      }

      let setor;

      if (username.includes(".")) {
        setor = (await search(username)).toLowerCase();
      } else if (username.includes("term")) {
        setor = username.replace(/term/gi, "").trim();
      } else {
        setor = username.toUpperCase();
      }

      let exist = await Depart.findOne({ setor: setor });

      if (exist) {
        console.log("debug");
        let inactivity_threshold = await Depart.findOne(
          { setor: setor },
          { time: 1, _id: 0 }
        );

        console.log("debug", inactivity_threshold);

        responses.push({
          status: 200,
          time: inactivity_threshold?.time || 60 
        });
      } else {
        responses.push({
          status: 404,
          message: 'Setor não encontrado',
          time: 60
        });
      }
    } catch (err) {
      console.error("Erro ao processar item:", err);
      responses.push({ 
        status: 500, 
        message: "Erro interno ao processar este item",
        time: 60
      });
    }
  }

  if (data.length === 1) {
    const response = responses[0];
    if (response.status !== 200) {
      return res.status(response.status).json(response);
    }
    return res.status(200).send(response.time.toString());
  } else {
    return res.status(200).json(responses);
  }
});


app.post('/dbinfos', verifyRequest, async (req, res) => {
  const data = Array.isArray(req.body) ? req.body : [req.body];
  const responses = [];

  try {
    for (const item of data) {
      const { 
        apiauth, nome, usuario, servicetag, modelo, versao,
        windows, ip, processador, graphiccard, ram, disco, monitor, snmonitor, time, programs 
      } = item;

      if (!servicetag || !apiauth) {
        responses.push({ status: 400, message: 'Autenticação obrigatoria!' });
        return;
      }

      if (!apiauth == password) {
        responses.push({ status: 401, message: 'Autenticação Falsa' });
        return;
      }
    
      try {

        let setor;

        if(usuario) {
          if(usuario.includes(".")) {
            setor = await search(usuario);
          } else if (usuario.includes("term")) {
            setor = usuario.replace(/term/gi, "".trim());
          } else {
            setor = usuario.toUpperCase();
          }
        } else {
          setor = 'Não informado'
        }
        const infoexist = await Infos.findOne({ servicetag: servicetag });
        let computerResponse;


        if (infoexist) {
          const updateInfo = await Infos.findOneAndUpdate(
            { servicetag },
            { 
              nome, usuario, modelo, versao,  
              windows, ip, processador, graphiccard, ram, disco, monitor, snmonitor, time, setor,
              aplicativos: processarListaProgramas(programs),
              ultimaAtualizacao: new Date()
            },
            { new: true }
          );
          computerResponse = { status: 200, data: updateInfo, message: 'Computador atualizado com sucesso' };
        } else {
          const newinfo = new Infos({
            nome, usuario, servicetag, modelo, versao, 
            windows, ip, processador, graphiccard, ram, disco, monitor, snmonitor, time, setor,
            aplicativos: processarListaProgramas(programs),
            dataCriacao: new Date(),
            ultimaAtualizacao: new Date()
          });
          await newinfo.save();
          computerResponse = { status: 201, data: newinfo, message: 'Novo computador registrado com sucesso' };
        }

        if (usuario) {
            try {
            let userDoc = await User.findOne({ username: usuario });
            const listaAplicativos = processarListaProgramas(programs);

            if (!userDoc) {
              userDoc = new User({
                username: usuario,
                setor: setor,
                aplicativos: listaAplicativos,
                pages: [],
              });
              await userDoc.save();
              console.log(`Novo usuário criado: ${usuario} do setor ${setor}`);
            } else {
              userDoc.setor = setor;
              userDoc.ultimaAtualizacao = new Date();
              
              if (listaAplicativos.length > 0) {
                const programasExistentes = new Set(userDoc.aplicativos || []);
                listaAplicativos.forEach(prog => {
                  if (!programasExistentes.has(prog)) {
                    userDoc.aplicativos.push(prog);
                  }
                });
              }
              
              await userDoc.save();
              console.log(`Usuário atualizado: ${usuario} do setor ${setor}`);
            }
            computerResponse.userInfo = {
              created: !userDoc,
              setor: setor,
              aplicativos: listaAplicativos.length
            };
            
          } catch (userErr) {
            console.error(`Erro ao criar/atualizar documento do usuário ${usuario}: ${userErr.message}`);
            computerResponse.userError = userErr.message;
          }
        }
        
        responses.push(computerResponse);
        
      } catch (err) {
        console.error(`Erro no processamento de informações para o computador: 
          Service Tag: ${servicetag}
          IP: ${ip}
          Usuário: ${usuario}
          Erro: ${err.message}`);
        
        responses.push({ 
          status: 500, 
          message: 'Erro ao processar a requisição.',
          error: err.message,
          computerInfo: { servicetag, ip, usuario }
        });
      }
    }

    return res.status(200).json({
      success: true,
      totalProcessed: data.length,
      responses
    });
    
  } catch (error) {
    console.error('Erro geral no processamento da requisição:', error);
    return res.status(500).json({
      success: false,
      message: 'Erro interno no servidor',
      error: error.message
    });
  }
});

app.post('/atualizar-documentos', verifyRequest, async (req, res) => {
  const data = Array.isArray(req.body) ? req.body : [req.body];
  const responses = [];

  try {
    for (const item of data) {
      try {
        const { user, page, seconds, date } = item;

        if (!user || !page || !date) {
          responses.push({
            status: 400,
            message: "Dados incompletos: usuário, página e data são obrigatórios"
          });
          continue;
        }

        const parsedSeconds = parseFloat(seconds || 0);
if (isNaN(parsedSeconds)) {
  responses.push({
    status: 400,
    message: `Valor inválido para seconds: ${seconds}`
  });
  continue;
}

let userDoc = await User.findOne({ username: user });
if (!userDoc) {
  const setor = await buscarSetorLDAP(user);
  userDoc = new User({
    username: user,
    setor: setor,
    aplicativos: [],
    pages: [],
  });
  await userDoc.save();}

  const clientIp = (req.headers['x-forwarded-for'] || req.socket.remoteAddress || '').replace('::ffff:', '');

  const sanitizedPageName = page.replace(/\(\d+(\.\d+)?\)/g, ' ').trim();

  const normalizedDate = date.trim();

  console.log(`Procurando: "${sanitizedPageName}" na data "${normalizedDate}"`);

  const existingPageIndex = userDoc.pages.findIndex(p => {
  const existingPageName = (p.page || '').trim().replace(/\(\d+(\.\d+)?\)/g, ' ');
  const existingDate = (p.date || '').trim();

  if (!existingDate || existingDate === '' || !existingPageName || existingPageName === '') {
    return false;
  }

  const pageMatches = existingPageName === sanitizedPageName;
  const dateMatches = existingDate === normalizedDate;

  return pageMatches && dateMatches;
});

if (existingPageIndex >= 0) {
  const currentTime = parseFloat(userDoc.pages[existingPageIndex].time || 0);
  userDoc.pages[existingPageIndex].time = currentTime + parsedSeconds;
} else {
  userDoc.pages.push({
    page: sanitizedPageName,
    time: parsedSeconds,
    date: normalizedDate,
    ip: clientIp,
    _id: new mongoose.Types.ObjectId()
  });
}

await userDoc.save();

        responses.push({
          status: 200,
          message: `Página ${page} adicionada/atualizada para o usuário ${user}`,
          seconds: parsedSeconds
        });

      } catch (userErr) {
        console.error(`Erro ao atualizar documentos do usuário: ${userErr.message}`);
        responses.push({
          status: 500,
          message: `Erro ao atualizar documentos do usuário ${item.user || 'desconhecido'}`,
          error: userErr.message
        });
      }
    }

    return res.status(200).json({
      success: true,
      totalProcessed: data.length,
      responses
    });

  } catch (error) {
    console.error('Erro geral no processamento da requisição:', error);
    return res.status(500).json({
      success: false,
      message: 'Erro interno no servidor',
      error: error.message
    });
  }
});

app.use((err, req, res, next) => {
  console.error('Erro não tratado:', err);
  res.status(500).json({
    success: false,
    message: 'Erro interno no servidor',
    error: process.env.NODE_ENV === 'production' ? 'Um erro ocorreu' : err.message
  });
});

app.listen(port, () => {
  console.log(`Servidor rodando em http://${SERVER_ADDRESS}:${port}`);
  console.log(`Ambiente: ${process.env.NODE_ENV || 'desenvolvimento'}`);
});

process.on('SIGTERM', () => {
  console.log('Sinal SIGTERM recebido. Encerrando servidor...');
  mongoose.connection.close(() => {
    console.log('Conexão MongoDB fechada');
    process.exit(0);
  });
});