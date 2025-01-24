const express = require('express');
const mongoose = require('mongoose');
const Infos = require('./dbinfos');
const app = express();
const port = 3000;

app.use(express.json());

mongoose.connect('mongodb://localhost:27017/InfosPC')
   .then(() => console.log('Conectado ao Mongodb'))
   .catch((err) => console.error('Error ao conectar ao mongo', err));


   app.post('/dbinfos', async (req, res) => {
    const { passwordpost, nome, 
            nomeusuario, servicetag, modelo, versao, 
            windows, ip, processador, monitor, snmonitor, time } = req.body;


    console.log(nome, nomeusuario, servicetag, modelo, versao);

    if (!passwordpost) {
        return res.status(400).json("Password Invalid")
    }

    if (passwordpost != "JolyneTheCat1202.07") {
        return res.status(400).json("Incorrect Password")
    }

    try {
        const infoexist = await Infos.findOne({ servicetag });

        if (infoexist) {
            const updateInfo = await Infos.findOneAndUpdate (
                  { servicetag },
                  { nome,nomeusuario, modelo, versao, 
                  windows, ip, processador, monitor, snmonitor, time },
                  { new: true }
            );
            return res.status(200).json(updateInfo);
        }

      const newinfo = new Infos({ nome,nomeusuario, servicetag, modelo, versao, 
                                  windows, ip, processador, monitor, snmonitor, time });
      await newinfo.save();
      res.status(201).json(newinfo);  // Retorna o novo documento
    } catch (err) {
      console.error(err);
      res.status(500).json({ message: 'Erro ao processar a requisição.' });
}

});

// Inicia o servidor
app.listen(port, () => {
    console.log(`Servidor rodando em http://192.168.20.8:${port}`);
});