import ldapjs from 'ldapjs';


export default async function buscarSetorLDAP(loginUsuario) {
    if (!loginUsuario) {
        console.log('Login do usuário não informado');
        return 'Não informado';
    }
    if (loginUsuario.toLowerCase().includes('candeias')) {
        const officeLocation = loginUsuario.replace(/candeias/gi, '').trim();
        return officeLocation.toUpperCase();
    }
    if (!loginUsuario.toLowerCase().includes('.')) {
        return loginUsuario.toUpperCase();
    }
    const AD_URL = process.env.AD_SERVER;
    if (!AD_URL) {
        console.log('URL do AD não configurada');
        return 'Não informado';
    }
    return new Promise((resolve, reject) => {
        const client = ldapjs.createClient({
            url: AD_URL
        });
        const bindDN = process.env.AD_USER;
        const bindpassword = process.env.AD_PASSWORD;
        client.bind(bindDN, bindpassword, (err) => {
            if (err) {
                console.log('Erro na autenticação LDAP:', err);
                client.unbind();
                resolve('Não informado');
                return;
            }
            const searchBase = process.env.AD_SEARCHBASE;
            const searchOptions = {
                scope: 'sub',
                filter: `(&(objectClass=user)(|(userPrincipalName=${loginUsuario})(userPrincipalName=${loginUsuario}@*)(sAMAccountName=${loginUsuario})))`,
                attributes: ['cn', 'physicalDeliveryOfficeName']
            };
            
            console.log(`Buscando usuário com login: ${loginUsuario}`);
            
            client.search(searchBase, searchOptions, (err, res) => {
                if (err) {
                    console.log('Erro na busca LDAP:', err);
                    client.unbind();
                    resolve('Não informado');
                    return;
                }
                let encontrou = false;
                let officeLocation = 'Não informado';
                
                res.on('searchEntry', (entry) => {
                    encontrou = true;
                    try {
                        if (entry.object && entry.object.physicalDeliveryOfficeName) {
                            officeLocation = entry.object.physicalDeliveryOfficeName;
                            console.log(`Setor encontrado via entry.object: ${officeLocation}`);
                        } 
                        else if (entry.attributes) {
                            entry.attributes.forEach(attr => {
                                if (attr.type === 'physicalDeliveryOfficeName' && attr.values && attr.values.length > 0) {
                                    officeLocation = attr.values[0];
                                    console.log(`Setor encontrado via entry.attributes: ${officeLocation}`);
                                }
                            });
                        }
                    } catch (e) {
                        console.log('Erro ao processar resposta LDAP:', e);
                    }
                    
                    console.log(`Valor final do setor após processamento: ${officeLocation}`);
                });
                
                res.on('error', (err) => {
                    console.log('Erro durante a busca LDAP:', err.message);
                    client.unbind();
                    resolve('Não informado');
                });
                
                res.on('end', () => {
                    if (!encontrou) {
                        console.log(`Nenhum usuário LDAP encontrado com o login "${loginUsuario}"`);
                    } else {
                        console.log(`Busca LDAP finalizada. Setor encontrado: ${officeLocation}`);
                    }
                    client.unbind();
                    resolve(officeLocation);
                });
            });
        });
    });
}