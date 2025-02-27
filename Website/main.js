const express = require('express');
const { MongoClient } = require('mongodb');
const bodyParser = require('body-parser');
const path = require('path');

const app = express();
const PORT = 8080;
const HOST = '192.168.22.80';

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(express.static(path.join(__dirname, 'static')));

const mongoUrl = 'mongodb://agente:JolyneTheCat120207@192.168.1.99:27017/InfosPC';
const dbName = 'InfosPC';
let db;

async function connectToMongo() {
    try {
        const client = await MongoClient.connect(mongoUrl, { useNewUrlParser: true, useUnifiedTopology: true });
        console.log('Conectado ao MongoDB com sucesso');
        db = client.db(dbName);
        return client;
    } catch (err) {
        console.error('Erro ao conectar ao MongoDB:', err);
    }
}

let previousDocuments = {};

const fieldsToTrack = ['ram', 'disco'];

async function pollForChanges() {
    try {
        const collection = db.collection('infos');
        const targetCollection = db.collection('warnings');

        const documents = await collection.find({}).toArray();
        const changes = [];

        documents.forEach(async doc => {
            const previousDoc = previousDocuments[doc._id];
            let hasChanges = false;
            fieldsToTrack.forEach(field => {
                if (previousDoc && previousDoc[field] !== doc[field]) {
                    changes.push({
                        timestamp: new Date(),
                        Informação: field,
                        ValorAntigo: previousDoc[field],
                        NovoValor: doc[field],
                        usuario: doc.usuario,
                        servicetag: doc.servicetag,
                        message: `Mudança detectada no campo ${field}`
                    });
                    hasChanges = true;
                }
            });
            previousDocuments[doc._id] = doc;

            if (hasChanges) {
                await collection.updateOne({ service: doc.servicetag }, { $set: { warning: true } });
            } else {
                await collection.updateOne({ service: doc.servicetag }, { $set: { warning: false } });
            }
        });

        if (changes.length > 0) {
            await targetCollection.insertMany(changes);
            console.log(`Alterações detectadas e registradas: ${changes.length} mudanças`);
        }
    } catch (err) {
        console.error('Erro ao monitorar mudanças:', err);
    }
}

app.get('/', (req, res) => {
    res.redirect('/inventorypage.html');
});

app.get('/inventorypage.html', (req, res) => {
    res.sendFile(path.join(__dirname, 'static', 'inventorypage.html'));
});

app.get('/get_warnings/:servicetag', async (req, res) => {
    try {
        const servicetag = req.params.servicetag;
        const collection = db.collection('warnings');
        const documents = await collection.find({ servicetag }).toArray();
        res.json(documents);
    } catch (err) {
        res.status(500).json({ error: 'Erro ao buscar dados do MongoDB' });
    }
});

app.get('/get_user_pages/:username', async (req, res) => {
    try {
        console.log('Buscando páginas do usuário:', req.params.username);
        const username = req.params.username;
        const collection = db.collection('pages');
        const documents = await collection.find({ user: username }).toArray();
        console.log('Páginas encontradas:', documents);
        res.json(documents);
    } catch (err) {
        res.status(500).json({ error: 'Erro ao buscar dados do MongoDB' });
    }
});



app.get('/get_all_data', async (req, res) => {
    try {
        const collection = db.collection('infos');
        const documents = await collection.find({}).toArray();
        const uniqueServicetags = new Set();
        const uniqueDocuments = documents.filter(doc => {
            if (uniqueServicetags.has(doc.servicetag)) {
                return false;
            } else {
                uniqueServicetags.add(doc.servicetag);
                return true;
            }
        });

        res.json(uniqueDocuments);
    } catch (err) {
        res.status(500).json({ error: 'Erro ao buscar dados do MongoDB' });
    }
});

async function startServer() {
    const client = await connectToMongo();
    if (client) {
        // Iniciar o polling para monitorar mudanças a cada 10 segundos
        setInterval(pollForChanges, 10000);
    }
    app.listen(PORT, HOST, () => {
        console.log(`Servidor rodando em http://${HOST}:${PORT}`);
        console.log(`Acesse a página de inventário em http://${HOST}:${PORT}/inventorypage.html`);
    });
}

startServer();