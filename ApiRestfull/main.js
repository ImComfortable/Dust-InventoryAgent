const express = require('express');
const mongoose = require('mongoose');
const Infos = require('./dbinfos');
const app = express();
const port = 3000;

app.use(express.json());

mongoose.connect('mongodb://localhost:27017/InfosPC', {
    useNewUrlParser: true,
    useUnifiedTopology: true,
})
   .then(() => console.log('Conectado ao Mongodb'))
   .catch((err) => console.error('Error ao conectar ao mongo', err));


   app.post('/dbinfos', async (req, res) => {
    const { nome, nomeusuario, servicetag, modelo, versao, windows } = req.body;
    console.log(nome, nomeusuario, servicetag, modelo, versao);

    const infoexist = await Infos.findOne({ servicetag })

    if (infoexist) {
        return res.status(400).json("Sem alterações.")
    }

    const newinfo = new Infos({ nome, nomeusuario, servicetag, modelo, versao, windows });
    console.log(newinfo);

    try {
        await newinfo.save();
        res.status(201).json(newinfo);
    } catch (err) {
        console.error(err);
        res.status(500).json({ message: 'Erro ao salvar as informações.' });
    }
});

// Inicia o servidor
app.listen(port, () => {
    console.log(`Servidor rodando em http://localhost:${port}`);
});