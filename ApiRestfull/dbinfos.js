const mongoose = require('mongoose');

const infoschema = new mongoose.Schema({
    // Informações do Dispositivo
    nome: {
        type: String,
        required: true,
    },
    modelo: {
        type: String,
        required: true,
    },
    servicetag: {
        type: String,
        required: true,
        unique: true,
    },
    snmonitor: {
        type: String,
        required: true,
        default: "Sem Monitor",
        set: function(value) {
            if (!value || value.trim().length === 0) {
                return "Sem Monitor";
            }
            return value;
        }
    },
    monitor: {
        type: String,
        required: true,
        default: "Sem Monitor",
        set: function(value) {
            if (!value || value.trim().length === 0) {
                return "Sem Monitor";
            }
            return value;
        }
    },

    // Informações do Sistema Operacional
    windows: {
        type: String,
        required: true,
    },
    versao: {
        type: String,
        required: true,
    },
    processador: {
        type: String,
        required: true,
    },
    time: {
        type: String,
        required: true
    },

    // Informações de Rede
    usuario: {
        type: String,
        required: true,
        validate: {
            validator: function(value) {
                const forbiddenNames = ['candeias', 'admin', 'teste'];
                return !forbiddenNames.includes(value.toLowerCase());
            },
            message: 'Nome de usuário não permitido.'
        },
    },
    ip: {
        type: String,
        required: true,
    },
});

const info = mongoose.model("infos", infoschema);

module.exports = info;
