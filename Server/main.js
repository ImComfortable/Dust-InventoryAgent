const express = require('express');
const session = require('express-session');
const { MongoClient } = require('mongodb');
const bodyParser = require('body-parser');
const path = require('path');

const app = express();
const PORT = 8080;
const HOST = '192.168.22.80';

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(express.static(path.join(__dirname, 'static')));

//const mongoUrl = 'mongodb://agente:JolyneTheCat120207@192.168.1.99:27017/InfosPC';]
const mongoUrl = 'mongodb://localhost:27017';
const dbName = 'infosdb';
let db;

app.use(session({
    secret: "secret",
    resave: false,
    saveUninitialized: true,
    cookie: {
        maxAge: 3 * 60 * 1000
    }
}));

function verifyauth(req, res, next) {
    if (req.session.usuarioId) {
        return next();
    } else {
        return res.redirect('/login.html');
    }
}

app.get('/verificar-sessao', (req, res) => {
    if (req.session && req.session.usuarioId) {
      const tempoRestante = req.session.cookie.maxAge;
      res.json({ 
        autenticado: true, 
        expirando: tempoRestante < 2 * 60 * 1000
      });
    } else {
      res.json({ autenticado: false });
    }
  });

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
    res.redirect('/login.html');
});

app.get('/inventorypage.html',  verifyauth, (req, res) => {
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

app.post('/auth', async (req, res) => {
    let username = req.body.username;
    let password = req.body.password;

    console.log('Usuário:', username);
    console.log('Senha:', password);
    try {
        const collection = db.collection('logins');
        const auth = await collection.findOne({ usuario: username, senha: password });

        if (auth) {
            req.session.usuarioId = 1;
            req.session.username = req.body.username;
            req.session.userRole = auth.role || 'Sem permissão';
            console.log(auth.setor); 
            req.session.userSetor = auth.setor || 'Sem setor';   
            res.redirect('/inventorypage.html');
        } else {
            res.redirect('/login.html');
        }
    } catch (err) {
        console.error('Erro ao autenticar usuário:', err);
        res.status(500).json({ error: 'Erro ao autenticar usuário' });
    }
});

app.get('/user-info', verifyauth, (req, res) => {
    res.json({ username: req.session.username, role: req.session.userRole, setor: req.session.userSetor });
});

app.get('/logout', (req, res) => {
    req.session.destroy();
    res.redirect('/login.html');
});

app.get('/get_audits', async (req, res) => {
    try {
        const collection = db.collection('logins');
        const audits = await collection.find({}).toArray();
        res.json(audits);
    } catch (err) {
        res.status(500).json({ error: 'Erro ao buscar dados de auditoria' });
    }
});

app.get('/get_all_audict', async (req, res) => {
    try {
        const collection = db.collection('users');
        const documents = await collection.find({}).toArray();
        res.json(documents);
    } catch (err) {
        res.status(500).json({ error: 'Erro ao buscar dados de auditoria' });
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
        setInterval(pollForChanges, 10000);
    }
    app.listen(PORT, HOST, () => {
        console.log(`Servidor rodando em http://${HOST}:${PORT}`);
        console.log(`Acesse a página de inventário em http://${HOST}:${PORT}/inventorypage.html`);
    });
}

startServer();