import { Container, Row, Col } from 'react-bootstrap'
import { Feed } from "../../components/Feed"

export const Home = () => {
    return(
        <div className='align-items-center'>
            <Container>
                <Row>
                    <Col>
                        <Feed />
                    </Col>
                </Row>
            </Container>
        </div>
    )
}