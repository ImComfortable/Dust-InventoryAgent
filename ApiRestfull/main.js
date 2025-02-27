const express = require('express');
const mongoose = require('mongoose');
const { Infos, Pages } = require('./dbinfos');
const app = express();
const port = 3000;

const MONGODB_URI = 'mongodb://agente:JolyneTheCat120207@192.168.1.99:27017/InfosPC';
const POST_PASSWORD = 'SuperSecretPostPassword';
const SERVER_ADDRESS = '192.168.22.80';

app.use(express.json());

// Conexão com MongoDB
mongoose.connect(MONGODB_URI)
   .then(() => console.log('Conectado ao MongoDB'))
   .catch((err) => console.error('Erro ao conectar ao MongoDB', err));

app.post('/dbinfos', async (req, res) => {
    const data = Array.isArray(req.body) ? req.body : [req.body];
    const responses = [];

    for (const item of data) {
        const { 
            passwordpost, nome, usuario, servicetag, modelo, versao,
            windows, ip, processador, ram, disco, monitor, snmonitor, time 
        } = item;

        // Validação da senha
        if (!passwordpost) {
            responses.push({ status: 400, message: "Password Invalid" });
            continue;
        }

        if (passwordpost !== POST_PASSWORD) {
            responses.push({ status: 400, message: "Incorrect Password" });
            continue;
        }

        try {
            const infoexist = await Infos.findOne({ servicetag });

            if (infoexist) {
                const updateInfo = await Infos.findOneAndUpdate(
                    { servicetag },
                    { 
                        nome, usuario, modelo, versao,  
                        windows, ip, processador, ram, disco, monitor, snmonitor, time 
                    },
                    { new: true }
                );
                responses.push({ status: 200, data: updateInfo });
            } else {
                const newinfo = new Infos({
                    nome, usuario, servicetag, modelo, versao, 
                    windows, ip, processador, ram, disco, monitor, snmonitor, time 
                });
                await newinfo.save();
                responses.push({ status: 201, data: newinfo });
            }
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

app.post('/atualizar-documentos', async (req, res) => {
    const data = Array.isArray(req.body) ? req.body : [req.body];
    console.log('Recebido dados para atualizar documentos:', data);
    const responses = [];

    for (const item of data) {
        try {
            // Validação dos dados de entrada - observe os nomes dos campos alterados
            const { user, page, date, seconds } = item;
            
            if (!user || !page || !date || seconds === undefined) {
                responses.push({ 
                    status: 400, 
                    message: "Dados incompletos. Forneça user, page, date e seconds" 
                });
                continue;
            }

            // Use 'page' como 'title' e 'seconds' como 'time'
            const filter = { user, title: page, date };
            const timeValue = Number(seconds);
            
            if (isNaN(timeValue)) {
                responses.push({ 
                    status: 400, 
                    message: "O valor de seconds deve ser um número" 
                });
                continue;
            }

            // Busca usando modelo Mongoose
            const existingDoc = await Pages.findOne(filter);

            if (existingDoc) {
                // Atualização usando modelo Mongoose
                const existingDuration = existingDoc.seconds || 0;
                const newDuration = existingDuration + timeValue;

                await Pages.updateOne(
                    filter,
                    { $set: { seconds: newDuration, last_updated: new Date().toISOString() } }
                );
                
                console.log(`Documento atualizado: ${page} para o usuário ${user} na data ${date}`);
                responses.push({ status: 200, message: `Documento atualizado: ${page}` });
            } else {
                // Criação usando modelo Mongoose
                const newDoc = new Pages({
                    user,
                    title: page,
                    date,
                    seconds: timeValue,
                    last_updated: new Date().toISOString()
                });

                await newDoc.save();
                
                console.log(`Novo documento inserido: ${page} para o usuário ${user} na data ${date}`);
                responses.push({ status: 201, message: `Novo documento inserido: ${page}` });
            }
        } catch (err) {
            console.error(`Erro ao processar documento: ${err.message}`);
            responses.push({ 
                status: 500, 
                message: 'Erro interno do servidor ao processar o documento.',
                error: err.message
            });
        }
    }

    res.status(200).json(responses);
});
   
// Inicia o servidor
app.listen(port, () => {
    console.log(`Servidor rodando em http://${SERVER_ADDRESS}:${port}`);
});