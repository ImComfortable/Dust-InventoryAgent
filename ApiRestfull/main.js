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
    const { nome, nomeusuario, servicetag , modelo, versao} = req.body;
    console.log(nome, nomeusuario, servicetag, modelo, versao);
   
    try {
        const newinfo = new Infos({ nome, nomeusuario, servicetag, modelo, versao});
        console.log(newinfo)
        await newinfo.save();
        res.status(201).json(newinfo);
    } catch (err) {
        res.status(500).json({ message: 'Error ao subir as infos para a db'})
    }
});

app.listen(port, () => {
    console.log(`Servidor rodando http://localhost:${port}`)
});