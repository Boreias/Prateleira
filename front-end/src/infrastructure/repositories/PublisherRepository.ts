import { IPublisherRepository } from "../../domain/irepositories/IPublisherRepository";



export class PublisherRepository implements IPublisherRepository {
    getAllPublishers(): Promise<any> {
        throw new Error("Method not implemented.");
    }
    getPublisherById(id: number): Promise<any> {
        throw new Error("Method not implemented.");
    }
    getPublisherByName(name: string): Promise<any> {
        throw new Error("Method not implemented.");
    }
    savePublisher(publisher: any): Promise<void> {
        throw new Error("Method not implemented.");
    }
}