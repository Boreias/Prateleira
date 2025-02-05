import { Row, Col, Container, Image } from 'react-bootstrap'
import Icon from '../../../assets/icon.png'


export const Footer = () =>
{
    return (
        <div className='fixed-bottom bg-body-tertiary'>
            <Container>
                <Row className='mt-2'>
                    <Col xs={7}>
                        <Row>
                            <Col>
                                <Image src={Icon} rounded />
                            </Col>
                            <Col>
                                <p className='mt-5'>O Prateleira serve para você que quer organizar suas leituras e acompanhar os seus amigos nas mais distintas histórias que os livros tem a oferecer.</p>
                            </Col>
                        </Row>
                    </Col>
                    <Col>
                        <Row className='mt-5'>
                            <Col>
                                <a href='https://www.instagram.com/'><i className='fa-brands fa-instagram fa-2xl'></i></a>
                            </Col>
                            <Col>
                                <a href='https://www.youtube.com/'><i className="fa-brands fa-youtube fa-2xl"></i></a>
                            </Col>
                            <Col>
                                <a href='https://www.tiktok.com/pt-BR/'><i className="fa-brands fa-tiktok fa-2xl"></i></a>
                            </Col>
                            <Col>
                                <a href='https://twitter.com/'><i className="fa-brands fa-twitter fa-2xl"></i></a>
                            </Col>
                        </Row>
                    </Col>
                    <Col>
                        <ul className='mt-5'>
                            <li><a href='/faq'>FAQ</a></li>
                            <li><a href='/sobre'>Quem Somos</a></li>
                            <li><a href='/livro/cadastro'>Cadastrar Livro</a></li>
                            <li><a href='/autor/cadastro'>Cadastrar Autor</a></li>
                        </ul>
                    </Col>
                </Row>
            </Container>
        </div>
    )
}