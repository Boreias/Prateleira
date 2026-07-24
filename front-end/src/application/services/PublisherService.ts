import type { iPublisherRepository } from "@/domain/irepositories/iPublisherRepository";
import { Publisher } from "@/domain/entities/publisher";


export class PublisherService {
    constructor(private publisherRepository: iPublisherRepository) {}

    async createPublisher(publisherObject: any): Promise<any> {
        return await this.publisherRepository.createPublisher(publisherObject);
    }

    async getPublisherById(id: string): Promise<Publisher> {
        return await this.publisherRepository.getPublisherById(id);
    }

    async getPublisherByName(name: string, skip: number, page_size: number): Promise<Publisher[]> {
        return await this.publisherRepository.getPublisherByName(name, skip, page_size);
    }

    async getPublisherByBook(book_id: string, skip: number, page_size: number): Promise<Publisher> {
        return await this.publisherRepository.getPublisherByBook(book_id, skip, page_size);
    }

    async getPublishersByAuthor(author_id: string, skip: number, page_size: number): Promise<Publisher[]> {
        return await this.publisherRepository.getPublishersByAuthor(author_id, skip, page_size);
    }

    async getPublishersByGender(gender_id: string, skip: number, page_size: number): Promise<Publisher[]> {
        return await this.publisherRepository.getPublishersByGender(gender_id, skip, page_size);
    }

    async morePopularPublishers(skip: number, page_size: number): Promise<Publisher[]> {
        return await this.publisherRepository.morePopularPublishers(skip, page_size);
    }

    async bestValuatedPublishers(skip: number, page_size: number): Promise<Publisher[]> {
        return await this.publisherRepository.bestValuatedPublishers(skip, page_size);
    }

    async alterPublisher(id: string, publisherObject: any): Promise<any> {
        return await this.publisherRepository.alterPublisher(id, publisherObject);
    }

    async deletePublisher(id: string, user_id: string): Promise<any> {
        return await this.publisherRepository.deletePublisher(id, user_id);
    }

    async clearDeletedPublishers(): Promise<any> {
        return await this.publisherRepository.clearDeletedPublishers();
    }
}