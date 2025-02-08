import { IPublisherRepository } from "../../domain/irepositories/IPublisherRepository";
import { Publisher } from "../../domain/entities/Publisher";


export class PublisherService implements IPublisherRepository {
    constructor(private PublisherRepository: IPublisherRepository) {}

    async getAllPublishers(): Promise<Publisher[]> {
        return await this.PublisherRepository.getAllPublishers();
    }

    async getPublisherById(id: number): Promise<Publisher> {
        return await this.PublisherRepository.getPublisherById(id);
    }

    async getPublisherByName(name: string): Promise<Publisher> {
        return await this.PublisherRepository.getPublisherByName(name);
    }

    async savePublisher(publisher: Publisher): Promise<void> {
        await this.PublisherRepository.savePublisher(publisher);
    }
}