-- Dados necessários para a execução dos testes automatizados

-- Gêneros
INSERT INTO gender (id, name) VALUES ('ae7df38c-8328-4077-ad2c-9670f11a9aad', 'Fantasia');
INSERT INTO gender (id, name) VALUES ('d0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe', 'Aventura');
INSERT INTO gender (id, name) VALUES ('c26bbe9d-5026-44d4-a096-df60889f8e85', 'Ficção');
INSERT INTO gender (id, name) VALUES ('e930b438-9a28-4c4d-bdd5-cda3e6d3621c', 'Literatura Estrangeira');
INSERT INTO gender (id, name) VALUES ('25580202-27b8-4fc5-a13d-c5ff3c36ecf1', 'Informática e Tecnologia');
INSERT INTO gender (id, name) VALUES ('bd8d9a17-08e2-4e4a-9920-988648880f68', 'Não-Ficção');
INSERT INTO gender (id, name) VALUES ('a49752cb-5bbc-428a-9f7e-2e99863988ef', 'Administração');
INSERT INTO gender (id, name) VALUES ('9349c148-2233-4fa8-ab44-7e52faac9923', 'Economia, Finanças');
INSERT INTO gender (id, name) VALUES ('85cd90a6-2282-4eff-b115-a97b08db83c4', 'Negócios e Empreendimentos');


-- Autores
INSERT INTO author (id, name) VALUES ('cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0', 'J. R. R. Tolkien');
INSERT INTO author (id, name) VALUES ('d309d334-8662-46f0-885e-60ddcb3eec23', 'Michael Moorcock');
INSERT INTO author (id, name) VALUES ('132917ef-25ce-4a38-af6c-deea1c0c2a89', 'Eduardo Azevedo');
INSERT INTO author (id, name) VALUES ('333ffde4-9167-4eaf-b744-1a904a234f9b', 'Aura Conci');
INSERT INTO author (id, name) VALUES ('7bb4a07f-4654-4aeb-89cb-52481e13a44e', 'Cristina Vasconcelos');
INSERT INTO author (id, name) VALUES ('75c6d1fc-c42f-4f28-b3a3-2cb38c5405a9', 'Fabiana Leta');
INSERT INTO author (id, name) VALUES ('a761613f-2a43-41ee-b5a8-db237840223d', 'Frederick P. Brooks Jr');
INSERT INTO author (id, name) VALUES ('5afc5e47-e873-4125-abce-243d11be331b', 'Eric Evan');
INSERT INTO author (id, name) VALUES ('625a2b53-9852-464a-a83d-5f8b78da8782', 'Robert C. Martin');
INSERT INTO author (id, name) VALUES ('f7dc9b34-877f-4abc-84f3-4c85303538a5', 'Ursula K. Le Guin');
INSERT INTO author (id, name) VALUES ('9bd713ef-e834-40ed-aa57-68683558f010', 'Robert T. Kiyosaki');


-- Editoras
INSERT INTO publisher (
    id,
    name,
    site,
    email
) VALUES (
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    'Alta Books',
    'https://altabooks.com.br/',
    'altabooks@altabooks.com.br'
);

INSERT INTO publisher (
    id,
    name,
    site,
    email
) VALUES (
    'b8ad6e89-69d9-4104-867a-c131a0a08750',
    'HarperCollins Brasil',
    'https://harpercollins.com.br/',
    'faleconosco@harpercollins.com.br'
);

INSERT INTO publisher (
    id,
    name,
    site
) VALUES (
    'bb1c9fb1-d725-4ec7-b26f-4088a4201a9a',
    'Panini',
    'https://panini.com.br/'
);

INSERT INTO publisher (
    id,
    name,
    site
) VALUES (
    'f5260958-2ac9-4c5e-8d47-fbca14899d62',
    'Black Library',
    'https://www.blacklibrary.com/'
);

INSERT INTO publisher (
    id,
    name,
    site,
    email
) VALUES (
    '757ea9a1-ab74-454a-9391-4425c4eb9316',
    'Pipoca e Nanquim',
    'https://pipocaenanquim.com.br/',
    'loja@pipocaenanquim.com.br'
);


-- Livros
INSERT INTO book (
    id,
    title,
    publisher_id,
    volume,
    edition,
    publication_year,
    pages,
    language,
    isbn
) VALUES (
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'A Saga de Elric',
    '757ea9a1-ab74-454a-9391-4425c4eb9316',
    1,
    1,
    2024,
    700,
    'Português',
    '978-6554480871'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '484a2b5e-18af-40b3-992c-dba0b91633a7',
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'd309d334-8662-46f0-885e-60ddcb3eec23'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'f3ec8eac-989b-4a19-bf62-0608ca047b81',
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'ae7df38c-8328-4077-ad2c-9670f11a9aad'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '5e2007b7-af6d-4fb4-bd5d-0f8b564baeac',
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'd0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '9cf877fd-c8d9-4017-9aeb-edda0342755f',
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'c26bbe9d-5026-44d4-a096-df60889f8e85'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '428a2cfb-cdd2-4ede-977c-2f76565a3f26',
    '7ee9f9ba-78de-45b0-acdc-60358fd4cf7a',
    'e930b438-9a28-4c4d-bdd5-cda3e6d3621c'
);



INSERT INTO book (
    id,
    title,
    publisher_id,
    volume,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    '2579a8cd-e838-4972-85fe-dd2451050719',
    'Computação Gráfica',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    1,
    2022,
    352,
    'Português',
    '9786555208269',
    'A Computação Gráfica permite produzir no computador coisas ainda não existentes, indo além do visível e do concreto, e permitindo aplicações inovadoras e inúmeras oportunidades profissionais.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '489325ac-f8b8-4108-9991-9526e9f62ba1',
    '2579a8cd-e838-4972-85fe-dd2451050719',
    '132917ef-25ce-4a38-af6c-deea1c0c2a89'
);
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '3dec93ac-9a85-47c1-a8b0-be1d84d5088d',
    '2579a8cd-e838-4972-85fe-dd2451050719',
    '333ffde4-9167-4eaf-b744-1a904a234f9b'
);
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    'f3917fd5-a250-45c7-9063-e48cfb7ffbf0',
    '2579a8cd-e838-4972-85fe-dd2451050719',
    '7bb4a07f-4654-4aeb-89cb-52481e13a44e'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '2c85424a-1c9d-4ce2-9f41-7a63174a395a',
    '2579a8cd-e838-4972-85fe-dd2451050719',
    '25580202-27b8-4fc5-a13d-c5ff3c36ecf1'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'a74e0d45-86e0-4199-83e7-f8f72b5809f0',
    '2579a8cd-e838-4972-85fe-dd2451050719',
    'bd8d9a17-08e2-4e4a-9920-988648880f68'
);


INSERT INTO book (
    id,
    title,
    publisher_id,
    volume,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    'Computação Gráfica',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    2,
    2022,
    352,
    'Português',
    '978-65-5520-816-0',
    'Imagens digitais capturadas do mundo real são cada vez mais usadas, não só em Computação, mas no cotidiano de todos os seres humanos. Usá-las adequadamente é, a cada ano, mais importante e não mais restrito a especialistas.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '4ff98ebe-2c63-45df-9119-0f0e135ad0ec',
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    '132917ef-25ce-4a38-af6c-deea1c0c2a89'
);
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    'd0b525f8-35c4-40b2-8ea1-e0d08d83b462',
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    '333ffde4-9167-4eaf-b744-1a904a234f9b'
);
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '6a4e1cf6-97ab-4f2b-9c7b-893761021c09',
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    '75c6d1fc-c42f-4f28-b3a3-2cb38c5405a9'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '36b302c3-5477-4491-b7e6-b719fc6d33fe',
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    '25580202-27b8-4fc5-a13d-c5ff3c36ecf1'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'a8acd4ad-ad71-46ed-bd86-5039d4a67b15',
    'c3aa4431-d14e-47d0-8dc7-f744dcd6cc51',
    'bd8d9a17-08e2-4e4a-9920-988648880f68'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    '818d509d-5574-4a29-95d3-811b8b7547ea',
    'O Mítico Homem-Mês',
    'Ensaios sobre engenharia de software',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    1,
    2018,
    320,
    'Português',
    '978-8550802534',
    'Poucos livros sobre gestão de projetos de software têm sido tão influentes e perenes quanto O Mítico Homem-Mês. Com uma rica mistura de fatos sobre engenharia de software e opiniões que provocam a reflexão, Frederick Brooks oferece sua visão para todos aqueles que lidam com a gestão de projetos complexos.
Os ensaios deste livro vêm diretamente da experiência de Frederick como gerente de projetos para a família de computadores System/360 da IBM e, posteriormente, com o OS/360, seu massivo sistema de software. Passados 20 anos da publicação original, em 1975, Brooks revisou suas ideias originais, adicionando novos pensamentos e conselhos em uma edição especial de aniversário.

Agora, quase 35 anos depois da primeira edição, comemorando sua primeira versão em português, Brooks concede uma entrevista exclusiva sobre seu trabalho, que os leitores podem conferir nas primeiras páginas deste livro.

O argumento central de O Mítico Homem-Mês é o de que grandes projetos de programação sofrem de problemas de gestão cuja natureza difere dos projetos pequenos em função da divisão das tarefas; a integridade conceitual de um produto é um fator crítico em seu desenvolvimento; e que é difícil, mas possível, atingir tal integridade. Seu ensaio seminal de 1986, "Não existe bala de prata" também está aqui, complementado por um novo texto da edição de 1995, onde Brooks afirma que "Não haverá nenhuma bala de prata no intervalo de dez anos". Estes dez anos já se passaram e Brooks continua atual.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '08dfecb3-b8df-4b75-8c62-1dfa321d8eb6',
    '818d509d-5574-4a29-95d3-811b8b7547ea',
    'a761613f-2a43-41ee-b5a8-db237840223d'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'ccf5aaae-b3ed-41d8-a7d5-859e61c44864',
    '818d509d-5574-4a29-95d3-811b8b7547ea',
    '25580202-27b8-4fc5-a13d-c5ff3c36ecf1'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '73b043ff-c0bb-4557-b5c2-bba205ea89c3',
    '818d509d-5574-4a29-95d3-811b8b7547ea',
    'bd8d9a17-08e2-4e4a-9920-988648880f68'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    'bf37ab23-09aa-474c-b109-7c36c8e23e6b',
    'Domain-Driven Design',
    'Atacando As Complexidades No Coração Do Software',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    3,
    2016,
    528,
    'Português',
    '978-85-508-0065-3',
    'A comunidade de desenvolvimento de softwares reconhece que a modelagem de domínios é fundamental para o design de softwares. Através de modelos de domínios, os desenvolvedores de software conseguem expressar valiosas funcionalidades e traduzi-las em uma implementação de software que realmente atenda às necessidades de seus usuários. Mas, apesar de sua óbvia importância, existem poucos recursos práticos que explicam como incorporar uma modelagem de domínios eficiente no processo de desenvolvimento de softwares.

O Domain-Driven Design atende essa necessidade. Este não é um livro sobre tecnologias específicas. Ele oferece aos leitores uma abordagem sistemática com relação ao domain-driven design, ou DDD, apresentando um conjunto abrangente de práticas ideais de design, técnicas baseadas em experiências e princípios fundamentais que facilitam o desenvolvimento de projetos de software que enfrentam domínios complexos. Reunindo práticas de design e implementação, este livro incorpora vários exemplos baseados em projetos que ilustram a aplicação do design dirigido por domínios no desenvolvimento de softwares na vida real.

Com este livro em mãos, desenvolvedores orientados a objetos, analistas de sistema e designers terão a orientação de que precisam para organizar e concentrar seu trabalho, criar modelos de domínio valiosos e úteis, e transformar esses modelos em implementações de software duradouras e de alta qualidade.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '6face197-505d-476c-a9fa-8a42f8f01ed6',
    'bf37ab23-09aa-474c-b109-7c36c8e23e6b',
    '5afc5e47-e873-4125-abce-243d11be331b'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'afb295f4-d699-4e0f-9b86-d5521c605e7d',
    'bf37ab23-09aa-474c-b109-7c36c8e23e6b',
    '25580202-27b8-4fc5-a13d-c5ff3c36ecf1'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'f672708a-c574-4d6d-8179-7d824ebed0d6',
    'bf37ab23-09aa-474c-b109-7c36c8e23e6b',
    'bd8d9a17-08e2-4e4a-9920-988648880f68'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    '8a99df16-a07c-44cf-8890-1fd3732055b9',
    'Código Limpo',
    'Habilidades Práticas do Agile Software',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    1,
    2009,
    440,
    'Português',
    '978-85-7608-267-5',
    'Mesmo um código ruim pode funcionar. Mas se ele não for limpo, pode acabar com uma empresa de desenvolvimento. Perdem-se a cada ano horas incontáveis e recursos importantes devido a um código mal escrito. Mas não precisa ser assim.

O renomado especialista em software, Robert C. Martin, apresenta um paradigma revolucionário com Código limpo: Habilidades Práticas do Agile Software. Martin se reuniu com seus colegas do Mentor Object para destilar suas melhores e mais ágeis práticas de limpar códigos “dinamicamente” em um livro que introduzirá gradualmente dentro de você os valores da habilidade de um profissional de softwares e lhe tornar um programador melhor –mas só se você praticar.

Que tipo de trabalho você fará? Você lerá códigos aqui, muitos códigos. E você deverá descobrir o que está correto e errado nos códigos. E, o mais importante, você terá de reavaliar seus valores profissionais e seu comprometimento com o seu ofício.

Código limpo está divido em três partes. Na primeira há diversos capítulos que descrevem os princípios, padrões e práticas para criar um código limpo.

A segunda parte consiste em diversos casos de estudo de complexidade cada vez maior. Cada um é um exercício para limpar um código - transformar o código base que possui alguns problemas em um melhor e eficiente. A terceira parte é a compensação: um único capítulo com uma lista de heurísticas e “odores” reunidos durante a criação dos estudos de caso. O resultado será um conhecimento base que descreve a forma como pensamos quando criamos, lemos e limpamos um código.


Após ler este livro os leitores saberão:

✔ Como distinguir um código bom de um ruim

✔ Como escrever códigos bons e como transformar um ruim em um bom

✔ Como criar bons nomes, boas funções, bons objetos e boas classes

✔ Como formatar o código para ter uma legibilidade máxima

✔ Como implementar completamente o tratamento de erro sem obscurecer a lógica

✔ Como aplicar testes de unidade e praticar o desenvolvimento dirigido a testes

Este livro é essencial para qualquer desenvolvedor, engenheiro de software, gerente de projeto, líder de equipes ou analistas de sistemas com interesse em construir códigos melhores.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '0a6fa372-358c-43d1-bfb4-3ba9acdc66b1',
    '8a99df16-a07c-44cf-8890-1fd3732055b9',
    '625a2b53-9852-464a-a83d-5f8b78da8782'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'd0b55635-8e2d-4871-a70a-19864756cb4c',
    '8a99df16-a07c-44cf-8890-1fd3732055b9',
    '25580202-27b8-4fc5-a13d-c5ff3c36ecf1'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '216c0651-bd84-4bbd-9b54-ed763ea1a5fe',
    '8a99df16-a07c-44cf-8890-1fd3732055b9',
    'bd8d9a17-08e2-4e4a-9920-988648880f68'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'O Senhor dos Anéis',
    'A Sociedade do Anel',
    'b8ad6e89-69d9-4104-867a-c131a0a08750',
    1,
    2019,
    576,
    'Português',
    '978-8595084759',
    'O volume inicial de O Senhor dos Anéis, lançado originalmente em julho de 1954, foi o primeiro grande épico de fantasia moderno, conquistando milhões de leitores e se tornando o padrão de referência para todas as outras obras do gênero até hoje. A imaginação prodigiosa de J.R.R. Tolkien e seu conhecimento profundo das antigas mitologias da Europa permitiram que ele criasse um universo tão complexo e convincente quanto o mundo real.

A Sociedade do Anel começa no Condado, a região rural do oeste da Terra-média onde vivem os diminutos e pacatos hobbits. Bilbo Bolseiro, um dos raros aventureiros desse povo, cujas peripécias foram contadas em O Hobbit, resolve ir embora do Condado e deixa sua considerável herança nas mãos de seu jovem parente Frodo.

O mais importante legado de Bilbo é o anel mágico que costumava usar para se tornar invisível. No entanto, o mago Gandalf, companheiro de aventuras do velho hobbit, revela a Frodo que o objeto é o Um Anel, a raiz do poder demoníaco de Sauron, o Senhor Sombrio, que deseja escravizar todos os povos da Terra-média. A única maneira de eliminar a ameaça de Sauron é destruir o Um Anel nas entranhas da própria montanha de fogo onde foi forjado.

A revelação faz com que Frodo e seus companheiros hobbits Sam, Merry e Pippin deixem a segurança do Condado e iniciem uma perigosa jornada rumo ao leste. Ao lado de representantes dos outros Povos Livres que resistem ao Senhor Sombrio, eles formam a Sociedade do Anel.

Alguém uma vez disse que o mundo dos leitores de língua inglesa se divide entre os que já leram O Senhor dos Anéis e os que um dia lerão o livro. Com esta nova tradução da obra, o fascínio dessa aventura atemporal ficará ainda mais evidente para os leitores brasileiros, tanto os que já conhecem a saga como os que estão prestes a descobrir seu encanto.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '8405080e-4f6e-46bc-894c-d2941975aa8c',
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '5a8f0282-f36a-4aa2-a7a7-a764d47a0242',
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'ae7df38c-8328-4077-ad2c-9670f11a9aad'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '08bdb542-6aad-497b-a871-17c039026831',
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'd0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '38042871-b14f-44c1-9ec4-14dd3256b3f8',
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'c26bbe9d-5026-44d4-a096-df60889f8e85'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '3253c648-0170-4942-b206-a9e40d5f1a9e',
    'b3fa5dfa-8908-485a-97e8-5f093ee8fa8c',
    'e930b438-9a28-4c4d-bdd5-cda3e6d3621c'
);

INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'O Senhor dos Anéis',
    'As duas torres',
    'b8ad6e89-69d9-4104-867a-c131a0a08750',
    1,
    2019,
    464,
    'Português',
    '978-8595084766',
    'O segundo volume de O Senhor dos Anéis, mais importante épico de fantasia moderno, narra os caminhos separados seguidos pelos membros da Sociedade do Anel em sua luta para deter Sauron, o Senhor Sombrio da terra de Mordor, e destruir o Um Anel, no qual está contida a maior parte do poder do tirano demoníaco imaginado por J.R.R. Tolkien.

Um ataque-surpresa pôs fim à jornada conjunta da Sociedade do Anel. De um lado, o trio formado pelo elfo Legolas, pelo anão Gimli e por Aragorn, herdeiro da realeza dos Homens, tenta resgatar os jovens hobbits Merry e Pippin, capturados por guerreiros-órquicos. A busca pelos companheiros perdidos levará os três a confrontar os cavaleiros do reino de Rohan e o mago renegado Saruman, que também deseja o Um Anel para si.

Enquanto isso, do outro lado das montanhas, Frodo e Sam buscam uma maneira de entrar em Mordor e chegar até a montanha onde o Anel foi forjado, único lugar onde é possível destruí-lo. Para isso, acabam recebendo a ajuda de seu mais improvável aliado: Gollum, a criatura que chegou a ter o Anel sob seu poder durante centenas de anos e que ainda é devorada, em corpo e alma, pelo desejo de voltar a possuí-lo.

Com cenas que mesclam o heroico e o intimista, o sublime e o cômico, As Duas Torres abriga algumas das criações mais inesquecíveis da imaginação de J.R.R. Tolkien, como os gigantescos Ents e a cultura nobre e belicosa do povo de Rohan.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    'b2bd94f2-7a53-4cf1-9bbe-873bd50507ed',
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '70f3e98d-e945-419a-bd1a-4d4c55a5fb01',
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'ae7df38c-8328-4077-ad2c-9670f11a9aad'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '4c4c284e-7810-4942-b3c9-85587a852781',
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'd0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'a833bc04-e99d-430b-a5ad-71352134da95',
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'c26bbe9d-5026-44d4-a096-df60889f8e85'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '058819f9-dc69-4c11-b97c-3938e93c5468',
    'd4b5e8cd-ea8c-4e13-97ea-a27b4391d5ff',
    'e930b438-9a28-4c4d-bdd5-cda3e6d3621c'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'O Senhor dos Anéis',
    'O retorno do rei',
    'b8ad6e89-69d9-4104-867a-c131a0a08750',
    1,
    2019,
    528,
    'Português',
    '978-8595084773',
    'A guerra entre os Povos Livres da Terra-média e Sauron, o Senhor Sombrio da terra de Mordor, chega a seu clímax neste terceiro volume do romance O Senhor dos Anéis. As batalhas grandiosas que estão prestes a acontecer, no entanto, são apenas o pano de fundo para o verdadeiro drama: a missão quase suicida dos hobbits Frodo e Sam, que tentam destruir o Um Anel, fonte do poder de Sauron, infiltrando-se no coração do território do Inimigo.

Em O Retorno do Rei, acompanhamos o mago Gandalf e o hobbit Pippin em sua visita à a majestosa cidade de Minas Tirith, que já foi o principal baluarte dos Homens contra a ameaça de Sauron, mas que está prestes a sucumbir diante da força avassaladora do Senhor Sombrio. Outros membros da Sociedade do Anel se juntam a Aragorn, herdeiro da longa linhagem dos reis de Minas Tirith, na tentativa de evitar que a antiga capital do reino de Gondor seja destruída.

Nas fronteiras de Mordor, Sam resgata Frodo, e os dois hobbits partem para o último estágio de sua jornada rumo ao Monte da Perdição, uma jornada que testará os limites do corpo e da mente dos pequenos heróis. O livro inclui ainda numerosos apêndices, nos quais Tolkien explora detalhes da história, das línguas, dos alfabetos e até dos calendários de seu mundo ficcional.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    'fa87d6e8-0b99-42f1-bb42-b0212e53ce3d',
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '2eb2d569-442d-4d41-8f39-85c91387372f',
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'ae7df38c-8328-4077-ad2c-9670f11a9aad'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '4734c56d-27f8-466f-aaa7-0717a940dc75',
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'd0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'c6011c98-003b-46ad-b845-60ac70ed0297',
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'c26bbe9d-5026-44d4-a096-df60889f8e85'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '3beca9e6-0f52-4b29-8e3c-ea0e2859c661',
    '3dbd31ad-1967-4a1f-97cd-fcf05bfb4185',
    'e930b438-9a28-4c4d-bdd5-cda3e6d3621c'
);


INSERT INTO book (
    id,
    title,
    subtitle,
    publisher_id,
    edition,
    publication_year,
    pages,
    language,
    isbn,
    synopsis
) VALUES (
    'd923da5e-96fc-4560-8381-83ee469511f4',
    'Pai Rico, Pai Pobre',
    'O que os ricos ensinam a seus filhos sobre dinheiro',
    'acd9ec73-901f-45b8-b121-3c78ba845c61',
    1,
    2017,
    336,
    'Português',
    '978-8550801483',
    'Celebrando 20 anos como o livro n° 1 em finanças pessoais.

A escola prepara as crianças para o mundo real? Essa é a primeira pergunta com a qual o leitor se depara neste livro. O recado é ousado e direto: boa formação e notas altas não bastam para assegurar o sucesso de alguém. O mundo mudou; a maioria dos jovens tem cartão de crédito, antes mesmo de concluir os estudos, e nunca teve aula sobre dinheiro, investimentos, juros etc. Ou seja, eles vão para a escola, mas continuam financeiramente improficientes, despreparados para enfrentar um mundo que valoriza mais as despesas do que a poupança.

Para o autor, o conselho mais perigoso que se pode dar a um jovem nos dias de hoje é: “Vá para a escola, tire notas altas e depois procure um trabalho seguro.” O fato é que agora as regras são outras, e não existe mais emprego garantido para ninguém. Pai Rico, Pai Pobre demonstra que a questão não é ser empregado ou empregador, mas ter o controle do próprio destino ou delegá-lo a alguém. É essa a tese de Robert Kiyosaki neste livro substancial e visionário. Para ele, a formação proporcionada pelo sistema educacional não prepara os jovens para o mundo que encontrarão depois de formados.

E como os pais podem ensinar aos filhos o que a escola relega? Essa é outra das muitas perguntas que o leitor encontra em Pai Rico, Pai Pobre. Nesse sentido, a proposta do autor é facilitar a tarefa dos pais. Quem entende de contabilidade deve esquecer seus conhecimentos acadêmicos, pois muitas das teorias expostas por Robert Kiyosaki contrariam os princípios contábeis comumente aceitos, e apresentam uma valiosa e moderna percepção do modo como se realizam os investimentos.

A sociedade sofre mudanças radicais e, talvez, de proporções maiores do que as ocorridas em séculos passados. Não existe bola de cristal, mas algo é certo: a perspectiva global de transformações transcende nossa realidade imediata. Aconteça o que acontecer, só existem duas alternativas: segurança ou independência financeira. E o objetivo de Pai Rico, Pai Pobre é instruir o leitor e despertar sua inteligência financeira e a de seus filhos.

“A principal razão pela qual as pessoas têm problemas financeiros é que passaram anos na escola, mas não aprenderam nada sobre dinheiro. O resultado é que elas aprendem a trabalhar por dinheiro… mas nunca a fazê-lo trabalhar para elas.” - Robert Kiyosaki.'
);
-- Livros - Author
INSERT INTO book_author (
    id,
    book_id,
    author_id
) VALUES (
    '1205a9ef-adb0-43da-89ca-9bdbb45af9c4',
    'd923da5e-96fc-4560-8381-83ee469511f4',
    '9bd713ef-e834-40ed-aa57-68683558f010'
);
-- Livro - Gênero
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'e1c963b2-da19-4001-b428-ad867330cc70',
    'd923da5e-96fc-4560-8381-83ee469511f4',
    'a49752cb-5bbc-428a-9f7e-2e99863988ef'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    'af94916e-e2b4-4f6a-8ec0-62b7ed14e329',
    'd923da5e-96fc-4560-8381-83ee469511f4',
    '9349c148-2233-4fa8-ab44-7e52faac9923'
);
INSERT INTO book_gender(
    id,
    book_id,
    gender_id 
) VALUES (
    '22a720f0-c9fe-4f25-b6a3-e5f35e253330',
    'd923da5e-96fc-4560-8381-83ee469511f4',
    '85cd90a6-2282-4eff-b115-a97b08db83c4'
);