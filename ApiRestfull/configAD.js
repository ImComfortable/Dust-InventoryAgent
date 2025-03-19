import { createClient } from 'ldapjs';

async function buscarSetorLDAP(loginUsuario) {
    return new Promise((resolve, reject) => {
      const client = createClient({
        url: process.env.AD_URL
      });
  
      const bindDN = process.env.AD_USER;
      const bindpassword = process.env.AD_PASSWORD;
  
      client.bind(bindDN, bindpassword, (err) => {
        if (err) {
          console.log('Erro na autenticação LDAP:', err);
          resolve('Não informado');
          return;
        }
        
        const searchBase = process.env.AD_SEARCHBASE;
        
        const searchOptions = {
          scope: 'sub',
          filter: `(&(objectClass=user)(|(userPrincipalName=*${loginUsuario}*)(sAMAccountName=*${loginUsuario}*)))`,
          attributes: ['cn', 'physicalDeliveryOfficeName']
        };
        
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
              if (entry.attributes) {
                entry.attributes.forEach(attr => {
                  if (attr.type === 'physicalDeliveryOfficeName') {
                    officeLocation = attr.values[0] || "Não informado";
                  }
                });
              }
              
              console.log(`setor localizado para ${loginUsuario}: ${officeLocation}`);
              
            } catch (e) {
              console.log('Erro ao processar resposta LDAP:', e.message);
            }
          });
          
          res.on('error', (err) => {
            console.log('Erro durante a busca LDAP:', err.message);
          });
          
          res.on('end', () => {
            if (!encontrou) {
              console.log(`Nenhum usuário LDAP encontrado com o login "${loginUsuario}"`);
            }
            client.unbind();
            resolve(officeLocation);
          });
        });
      });
    });
  }