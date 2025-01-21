const mongoose = require('mongoose');

const infoschema = new mongoose.Schema({
    nome: {
        type: String,
        required: true,
    },
    username: {
        type: String,
        required: true,
    },
    servicetag: {
        type: String,
        required: true,
        unique: true,
    },
    modelo: {
        type: String,
        required: true,
    },
    versao: {
        type: String,
        required: true
    },
});

const info = mongoose.model("infos", infoschema);

module.exports = info;