const express = require('express');
const mongoose = require('mongoose');
const Infos = require('./dbinfos');
const app = express();
const port = 3000;

app.use(express.json());

mongoose.connect('mongodb://agent:JolyneTheCat%211202%2E07@192.168.1.99:27017/InfosPC?authSource=InfosPC', {
    useNewUrlParser: true,
    useUnifiedTopology: true,
})
   .then(() => console.log('Conectado ao Mongodb'))
   .catch((err) => console.error('Error ao conectar ao mongo', err));

app.post('/dbinfos', async (req, res) => {
    const { nome, username, servicetag , modelo, versao} = req.body;
   
    try {
        const newinfo = new Infos({ nome, username, servicetag, modelo, versao});
        await newinfo.save();
        res.status(201).json(newinfo);
    } catch (err) {
        res.status(500).json({ message: 'Error ao subir as infos para a db'})
    }
});

app.listen(port, () => {
    console.log(`Servidor rodando http://localhost:${port}`)
});