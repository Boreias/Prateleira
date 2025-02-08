import { Button, Col, Row, Form, Nav, Navbar, Container } from 'react-bootstrap';


export const Header = () => 
{
    return(
        <Navbar expand="lg" fixed='top' className='bg-body-tertiary'>
            <Container>
                <Navbar.Brand href='/'>Prateleira</Navbar.Brand>
                <Navbar.Toggle aria-controls="basic-navbar-nav" />
                <Navbar.Collapse id="basic-navbar-nav">
                    <Form className='me-auto'>
                        <Row>
                            <Col xs='auto'>
                                <Form.Control
                                    type='text'
                                    placeholder='Pesquise por título, autor, editora ou ISBN'
                                    className='mr-sm-2'
                                />
                            </Col>
                            <Col xs='auto'>
                                <Button type='submit'>Pesquisar</Button>
                            </Col>
                        </Row>
                    </Form>
                    <Nav className='me-auto'>
                        <Nav.Link href='/'>Home</Nav.Link>
                        <Nav.Link href='/'>Sobre</Nav.Link>
                        <Nav.Link href='/'>FAQ</Nav.Link>
                        <Nav.Link href='/'>Cadastrar Livro</Nav.Link>
                        <Nav.Link href='/'>Cadastrar Autor</Nav.Link>
                    </Nav>
                    <Button className='btn btn-primary' href='/login'>Entrar</Button>
                </Navbar.Collapse>
            </Container>
        </Navbar>
    )
}