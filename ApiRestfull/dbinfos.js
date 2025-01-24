const mongoose = require('mongoose');

const infoschema = new mongoose.Schema({
    nome: {
        type: String,
        required: true,
    },
    nomeusuario: {
        type: String,
        required: true,
        validate: {
            validator: function(value){
            return value.toLowerCase() !== 'candeias';
            },
        },
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
        required: true,
    },
    windows: {
        type: String,
        required: true,
    },
    ip: {
        type: String,
        required: true,
    },
    processador: {
        type: String,
        required: true,
    },
    monitor: {
        type: String,
        required: true
    },
    snmonitor: {
        type: String,
        required: true
    },
    time: {
        type: String,
        required: true
    },
});

const info = mongoose.model("infos", infoschema);

module.exports = info;