const express = require('express');
const mongoose = require('mongoose');
const ldap = require('ldapjs');
const { Infos, User } = require('./dbinfos');
const app = express();
const port = 3000;

const MONGODB_URI = 'mongodb://localhost:27017/infosdb';
const POST_PASSWORD = 'SuperSecretPostPassword';
const SERVER_ADDRESS = '192.168.22.80';

app.use(express.json());

// Conexão com MongoDB
mongoose.connect(MONGODB_URI)
   .then(() => console.log('Conectado ao MongoDB'))
   .catch((err) => console.error('Erro ao conectar ao MongoDB', err));

async function buscarSetorLDAP(loginUsuario) {
    return new Promise((resolve, reject) => {
      const client = ldap.createClient({
        url: 'ldap://192.168.1.27'
      });
  
      const bindDN = "CN=Carlos Eduardo Lussoli,OU=Teste,OU=Filial Itajaí,DC=candeias,DC=tur,DC=local";
      const bindpassword = "";
  
      client.bind(bindDN, bindpassword, (err) => {
        if (err) {
          console.log('Erro na autenticação LDAP:', err);
          resolve('Não informado'); // Em caso de erro, retornamos um valor padrão
          return;
        }
        
        const searchBase = 'DC=candeias,DC=tur,DC=local';
        
        const searchOptions = {
          scope: 'sub',
          filter: `(&(objectClass=user)(|(userPrincipalName=*${loginUsuario}*)(sAMAccountName=*${loginUsuario}*)))`,
          attributes: ['cn', 'physicalDeliveryOfficeName']
        };
        
        client.search(searchBase, searchOptions, (err, res) => {
          if (err) {
            console.log('Erro na busca LDAP:', err);
            client.unbind();
            resolve('Não informado');
            return;
          }
          
          let encontrou = false;
          let officeLocation = 'Não informado';
          
          res.on('searchEntry', (entry) => {
            encontrou = true;
            
            try {
              if (entry.attributes) {
                entry.attributes.forEach(attr => {
                  if (attr.type === 'physicalDeliveryOfficeName') {
                    officeLocation = attr.values[0] || "Não informado";
                  }
                });
              }
              
              console.log(`setor localizado para ${loginUsuario}: ${officeLocation}`);
              
            } catch (e) {
              console.log('Erro ao processar resposta LDAP:', e.message);
            }
          });
          
          res.on('error', (err) => {
            console.log('Erro durante a busca LDAP:', err.message);
          });
          
          res.on('end', () => {
            if (!encontrou) {
              console.log(`Nenhum usuário LDAP encontrado com o login "${loginUsuario}"`);
            }
            client.unbind();
            resolve(officeLocation);
          });
        });
      });
    });
  }
  
  // Endpoint para cadastrar computadores modificado
  app.post('/dbinfos', async (req, res) => {
      const data = Array.isArray(req.body) ? req.body : [req.body];
      const responses = [];
  
      for (const item of data) {
          const { 
              passwordpost, nome, usuario, servicetag, modelo, versao,
              windows, ip, processador, ram, disco, monitor, snmonitor, time, programs 
          } = item;
  
          console.log(item);
  
          // Validação da senha (comentado como no código original)
          /*if (!passwordpost) {
              responses.push({ status: 400, message: "Password Invalid" });
              continue;
          }*/
  
          /*if (passwordpost !== POST_PASSWORD) {
              responses.push({ status: 400, message: "Incorrect Password" });
              continue;
          }*/
  
          try {
              const infoexist = await Infos.findOne({ servicetag: servicetag });
              let computerResponse;
  
              if (infoexist) {
                  const updateInfo = await Infos.findOneAndUpdate(
                      { servicetag: servicetag },
                      { 
                          nome, usuario, modelo, versao,  
                          windows, ip, processador, ram, disco, monitor, snmonitor, time 
                      },
                      { new: true }
                  );
                  computerResponse = { status: 200, data: updateInfo };
              } else {
                  const newinfo = new Infos({
                      nome, usuario, servicetag, modelo, versao, 
                      windows, ip, processador, ram, disco, monitor, snmonitor, time
                  });
                  await newinfo.save();
                  computerResponse = { status: 201, data: newinfo };
              }
              
              // Cadastrar ou atualizar o usuário junto com os programas
              if (usuario) {
                try {
                    // Buscar setor do usuário via LDAP
                    const setor = await buscarSetorLDAP(usuario);
                    
                    // Verifica se o usuário já existe
                    let userDoc = await User.findOne({ username: usuario });
                    
                    // Preparar a lista de aplicativos - SOLUÇÃO SIMPLES
                    let listaAplicativos = [];
                    
                    // Se programs existe
                    if (programs) {
                        // Se já é um array, usar diretamente
                        if (Array.isArray(programs)) {
                            // Extrair apenas os nomes dos programas
                            listaAplicativos = programs.map(p => 
                                typeof p === 'string' ? p : (p.nome || String(p))
                            );
                        } 
                        // Se é uma string, tentar fazer parse
                        else if (typeof programs === 'string' && programs.trim().startsWith('[')) {
                            try {
                                // Parse do JSON e extrair os nomes
                                const parsedPrograms = JSON.parse(programs);
                                listaAplicativos = parsedPrograms.map(p => p.nome || String(p));
                            } catch (e) {
                                console.log('Erro ao processar JSON de programas:', e.message);
                                // Em caso de erro, usar string bruta
                                listaAplicativos = [String(programs)];
                            }
                        }
                        // Outro caso
                        else {
                            listaAplicativos = [String(programs)];
                        }
                    }
                    
                    if (!userDoc) {
                        // Se não existir, cria um novo documento
                        userDoc = new User({
                            username: usuario,
                            setor: setor,
                            aplicativos: listaAplicativos,
                            pages: []
                        });
                        await userDoc.save();
                        console.log(`Novo usuário criado: ${usuario} do setor ${setor}`);
                    } else {
                        // Se existir, atualiza o setor e adiciona novos programas
                        userDoc.setor = setor;
                        
                        // Adicionar novos programas sem duplicação
                        if (listaAplicativos.length > 0) {
                            // Converter array existente para Set para verificação rápida
                            const programasExistentes = new Set(userDoc.aplicativos || []);
                            
                            // Adicionar apenas programas que não existem
                            listaAplicativos.forEach(prog => {
                                if (!programasExistentes.has(prog)) {
                                    userDoc.aplicativos.push(prog);
                                }
                            });
                        }
                        
                        userDoc.updated_at = new Date();
                        await userDoc.save();
                        console.log(`Usuário atualizado: ${usuario} do setor ${setor}`);
                    }
                    
                    // Adiciona informação de sucesso
                    computerResponse.userCreated = true;
                    computerResponse.userSetor = setor;
                    
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
                computerInfo: { servicetag, ip, usuario }
            });
        }
    }

    res.status(200).json(responses);
});
  
  // Endpoint para atualizar documentos modificado
  app.post('/atualizar-documentos', async (req, res) => {
    const data = Array.isArray(req.body) ? req.body : [req.body];
    console.log('Recebido dados para atualizar documentos:', data);
    const responses = [];

    for (const item of data) {
        try {
            const { user, page, date, seconds } = item;
            // Primeiro, verifica se o usuário existe
            let userDoc = await User.findOne({ username: user });

            if (!userDoc) {
                // Buscar setor via LDAP se o usuário não existir
                const setor = await buscarSetorLDAP(user);

                // Cria o usuário se não existir
                userDoc = new User({
                    username: user,
                    setor: setor,
                    aplicativos: [],
                    pages: []
                });
            }

            // Procura se a página já existe no array pelo título
            const pageIndex = userDoc.pages.findIndex(p => p.title === page);

            if (pageIndex >= 0) {
                // Se a página existe, atualiza seus dados
                const existingSeconds = userDoc.pages[pageIndex].seconds || 0;
                userDoc.pages[pageIndex].seconds = existingSeconds + seconds;
                userDoc.pages[pageIndex].date = date;  // Atualiza a data se necessário
                userDoc.pages[pageIndex].last_updated = new Date();
            } else {
                // Se não existe, adiciona a nova página ao array
                userDoc.pages.push({
                    title: page,
                    date: date,
                    seconds: seconds,
                    last_updated: new Date()
                });
            }

            userDoc.updated_at = new Date();
            await userDoc.save();

            console.log(`Página ${page} adicionada/atualizada para o usuário ${user}`);
            responses.push({ status: 200, message: `Página ${page} adicionada/atualizada para o usuário ${user}` });

        } catch (userErr) {
            console.error(`Erro ao atualizar documentos do usuário: ${userErr.message}`);
            responses.push({ 
                status: 500, 
                message: `Erro ao atualizar documentos do usuário ${item.user || 'desconhecido'}`, 
                error: userErr.message 
            });
        }
    }

    res.status(200).json(responses);
});
  
  // Endpoint adicional para buscar dados do usuário
  app.get('/usuario/:username', async (req, res) => {
      try {
          const { username } = req.params;
          
          // Buscar usuário no banco de dados
          const usuario = await User.findOne({ username });
          
          if (!usuario) {
              // Se não encontrar, tentar buscar o setor via LDAP
              const setor = await buscarSetorLDAP(username);
              
              if (setor !== 'Não informado') {
                  // Criar novo usuário com o setor encontrado
                  const novoUsuario = new User({
                      username,
                      setor,
                      aplicativos: [],
                      pages: []
                  });
                  
                  await novoUsuario.save();
                  res.status(201).json({
                      message: "Usuário criado com informações do LDAP",
                      data: novoUsuario
                  });
              } else {
                  res.status(404).json({ message: "Usuário não encontrado" });
              }
          } else {
              // Se encontrar, retornar os dados
              res.status(200).json({
                  message: "Usuário encontrado",
                  data: usuario
              });
          }
      } catch (err) {
          console.error(`Erro ao buscar usuário: ${err.message}`);
          res.status(500).json({
              message: "Erro ao buscar usuário",
              error: err.message
          });
      }
  });
   
// Inicia o servidor
app.listen(port, () => {
    console.log(`Servidor rodando em http://${SERVER_ADDRESS}:${port}`);
});


