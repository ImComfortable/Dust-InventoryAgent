import { date } from 'joi';
import { Schema, model, set } from 'mongoose';

const infoschema = new Schema({
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
    graphiccard: {
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
    setor: {
        type: String,
        required: true,
    },
    ip: {
        type: String,
        required: true,
    },
    aplicativos: {
        type: [String],
        default: []
      },
});

const UserSchema = new Schema({
    username: { 
      type: String, 
      required: true,
      unique: true
    },
    setor: {
      type: String,
      default: 'Não informado'
    },
    pages: [
        {
            page: { type: String, required: true },
            time: { type: Number, default: 0 },
            date: { type: String, required: true},
            horario: { type: String, default: () => {
                let date = new Date();
                let hours = date.getHours().toString().padStart(2, '0');
                let minutes = date.getMinutes().toString().padStart(2, '0');
                return `${hours}:${minutes}`;
            }}
        }
    ],
  }); 

const Infos = model("infos", infoschema);
const User = model("user", UserSchema);

export default {
    Infos,
    User
}