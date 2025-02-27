const mongoose = require('mongoose');

const infoschema = new mongoose.Schema({
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
    ram: {
        type: String,
        required: true,
    },
    disco: {
        type: String,
        required: true,
    },
    time: {
        type: String,
        required: true
    },

    usuario: {
        type: String,
        required: true,
        validate: {
            validator: function(value) {
                const forbiddenNames = ['admin', 'teste'];
                return !forbiddenNames.includes(value.toLowerCase());
            },
            message: 'Nome de usuário não permitido.'
        },
    },
    ip: {
        type: String,
        required: true,
    },
    programs: {
        type: Array,
        required: true,
    },
});

const pagesSchema = new mongoose.Schema({
    user: {
        type: String,
        required: true,
    },
    title: {
        type: String,
        required: true,
    },
    date: {
        type: String,
        required: true,
    },
    seconds: {
        type: Number,
        required: true,
    },
})

const Info = mongoose.model("infos", infoschema);
const Pages = mongoose.model("pages", pagesSchema);

module.exports = {
    Info,
    Pages
}