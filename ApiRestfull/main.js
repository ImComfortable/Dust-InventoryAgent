const express = require('express');
const mongoose = require('mongoose');
const Infos = require('./dbinfos');
const app = express();
const port = 3000;

app.use(express.json());

mongoose.connect('mongodb://mongoconnectURL')
   .then(() => console.log('Conectado ao Mongodb'))
   .catch((err) => console.error('Error ao conectar ao mongo', err));

   app.post('/dbinfos', async (req, res) => {
    const data = Array.isArray(req.body) ? req.body : [req.body];

    const responses = [];

    for (const item of data) {
        const { passwordpost, nome, usuario, servicetag, modelo, versao, 
                windows, ip, processador, monitor, snmonitor, time } = item;

        if (!passwordpost) {
            responses.push({ status: 400, message: "Password Invalid" });
            continue;
        }

        if (passwordpost !== "SuperSecretPostPassword") {
            responses.push({ status: 400, message: "Incorrect Password" });
            continue;
        }

        try {
            const infoexist = await Infos.findOne({ servicetag });

            if (infoexist) {
                const updateInfo = await Infos.findOneAndUpdate(
                    { servicetag },
                    { nome, usuario, modelo, versao,  
                      windows, ip, processador, monitor, snmonitor, time },
                    { new: true }
                );
                responses.push({ status: 200, data: updateInfo });
            } else {
                const newinfo = new Infos({
                    nome, usuario, servicetag, modelo, versao, 
                    windows, ip, processador, monitor, snmonitor, time 
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

// Inicia o servidor
app.listen(port, () => {
    console.log(`Servidor rodando em http://192.168.1.99:${port}`);
});
