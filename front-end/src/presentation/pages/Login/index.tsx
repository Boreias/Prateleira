import { Form, Row, Col, FloatingLabel, Button, Container } from 'react-bootstrap';


export const Login = () => {
    return(
        <div>
            <Container>
                <Form>
                    <Row className='justify-content-md-center'>
                        <h1>Formulário de Cadastro</h1>
                    </Row>
                    <Row className='justify-content-md-center mt-5'>
                        <Form.Group controlId='avatarId'>
                            <Form.Label>Avatar</Form.Label>
                            <Form.Control type='file' />
                        </Form.Group>
                    </Row>

                    <Row className='justify-content-md-center g-3 mt-2'>
                        <Col md>
                            <FloatingLabel controlId='nameId' label="Nome">
                                <Form.Control type='text' placeholder='Nome' />
                            </FloatingLabel>
                        </Col>

                        <Col md>
                            <FloatingLabel controlId='nicknameId' label="Apelido">
                                <Form.Control type='text' placeholder='Apelido' />
                            </FloatingLabel>
                        </Col>

                        <Col md>
                            <FloatingLabel controlId='birthDataId' label="Data de Nascimento">
                               <Form.Control type='date' placeholder='01/01/2000' />
                            </FloatingLabel>
                        </Col>
                    </Row>

                    <Row className='justify-content-md-center mt-2'>
                        <FloatingLabel controlId='emailId' label='Email'>
                            <Form.Control type='email' placeholder='nome@exemplo.com' />
                        </FloatingLabel>
                    </Row>

                    <Row className='justify-content-md-center mt-2 g-2'>
                        <Col md>
                            <FloatingLabel controlId='passwordId' label='Senha'>
                                <Form.Control type='password' placeholder='Senha' />
                            </FloatingLabel>
                        </Col>

                        <Col md>
                            <FloatingLabel controlId='confirmPasswordId' label='Confirme a Senha'>
                                <Form.Control type='password' placeholder='Confirme a Senha' />
                            </FloatingLabel>
                        </Col>
                    </Row>

                    <Button className='mt-2' type='submit'>Enviar</Button>
                </Form>
            </Container>
        </div>
    )
}