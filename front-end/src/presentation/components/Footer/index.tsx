import { Row, Col, Container, Image } from 'react-bootstrap'
import { UlStyled, AStyled } from '../../../styles'
import Icon from '../../../assets/icon.png'


export const Footer = () =>
{
    return (
        <div className='fixed-bottom bg-body-tertiary'>
            <Container>
                <Row className='mt-2'>
                    <Col xs={7}>
                        <Row className='align-items-center'>
                            <Col>
                                <Image src={Icon} width={128} height={128} rounded />
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
                        <UlStyled className='mt-5'>
                            <li><AStyled href='/livro/cadastro'>Cadastrar Livro</AStyled></li>
                            <li><AStyled href='/autor/cadastro'>Cadastrar Autor</AStyled></li>
                            <li><AStyled href='/sobre'>Quem Somos</AStyled></li>
                            <li><AStyled href='/faq'>FAQ</AStyled></li>
                        </UlStyled>
                    </Col>
                </Row>
            </Container>
        </div>
    )
}