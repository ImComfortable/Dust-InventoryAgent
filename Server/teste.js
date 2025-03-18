const ldap = require('ldapjs');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Digite o nome do usuário que deseja buscar: ', (loginUsuario) => {
  const client = ldap.createClient({
    url: 'ldap://192.168.1.27'
  });

  const bindDN = "CN=Carlos Eduardo Lussoli,OU=Teste,OU=Filial Itajaí,DC=candeias,DC=tur,DC=local";
  const bindpassword = "";

  client.bind(bindDN, bindpassword, (err) => {
    if (err) {
      console.log('Erro na autenticação:', err);
      rl.close();
      return;
    }
    
    //console.log('Autenticação bem-sucedida');
    
    const searchBase = 'DC=candeias,DC=tur,DC=local';
    
    const searchOptions = {
      scope: 'sub',
      filter: `(&(objectClass=user)(|(userPrincipalName=*${loginUsuario}*)(sAMAccountName=*${loginUsuario}*)))`,
      attributes: ['cn', 'physicalDeliveryOfficeName']
    };
    
    client.search(searchBase, searchOptions, (err, res) => {
      if (err) {
        console.log('Erro na busca:', err);
        rl.close();
        return;
      }
      
      let encontrou = false;
      
      res.on('searchEntry', (entry) => {
        encontrou = true;
        
        try {
          let userName = "Usuário";
          let officeLocation = "Não informado";
          
          if (entry.attributes) {
            entry.attributes.forEach(attr => {
              if (attr.type === 'cn') {
                userName = attr.values[0];
              }
              if (attr.type === 'physicalDeliveryOfficeName') {
                officeLocation = attr.values[0] || "Não informado";
              }
            });
          }
          
          console.log(`setor localizado: ${officeLocation}`);
          
        } catch (e) {
          console.log('Erro ao processar resposta:', e.message);
        }
      });
      
      res.on('error', (err) => {
        console.log('Erro durante a busca:', err.message);
      });
      
      res.on('end', (result) => {
        if (!encontrou) {
          console.log(`Nenhum usuário encontrado com o nome "${nomeUsuario}"`);
        }
        client.unbind();
        rl.close();
      });
    });
  });
});