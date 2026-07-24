import type { iPublisherRepository } from "@/domain/irepositories/iPublisherRepository";
import { Publisher } from "@/domain/entities/publisher";
import api from "../api/api";


export class PublisherRepository implements iPublisherRepository {
    private readonly apiUrl = "publisher/";

    async createPublisher(publisherObject: any): Promise<any> {
        const response = await api.post(`${this.apiUrl}create`, publisherObject);

        return response;
    }

    async getPublisherById(id: string): Promise<Publisher> {
        const response = await api.get(`${this.apiUrl}id?id=${id}`);

        return new Publisher(
            response.data.id,
            response.data.name,
            response.data.site,
            response.data.email,
            response.data.avatar,
            response.data.books
        );
    }

    async getPublisherByName(name: string, skip: number = 0, page_size: number = 20): Promise<Publisher[]> {
        const response = await api.get(`${this.apiUrl}name?name=${name}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async getPublisherByBook(book_id: string, skip: number = 0, page_size: number = 20): Promise<Publisher> {
        const response = await api.get(`${this.apiUrl}book?book_id=${book_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async getPublishersByAuthor(author_id: string, skip: number = 0, page_size: number = 20): Promise<Publisher[]> {
        const response = await api.get(`${this.apiUrl}author?author_id=${author_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async getPublishersByGender(gender_id: string, skip: number = 0, page_size: number = 20): Promise<Publisher[]> {
        const response = await api.get(`${this.apiUrl}gender?gender_id=${gender_id}&skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async morePopularPublishers(skip: number = 0, page_size: number = 20): Promise<Publisher[]> {
        const response = await api.get(`${this.apiUrl}more_popular?skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async bestValuatedPublishers(skip: number = 0, page_size: number = 20): Promise<Publisher[]> {
        const response = await api.get(`${this.apiUrl}best_valuated?skip=${skip}&page_size=${page_size}`);

        return response.data.map((publisher: any) => new Publisher(
            publisher.id,
            publisher.name,
            publisher.site,
            publisher.email,
            publisher.avatar,
            publisher.books
        ));
    }

    async alterPublisher(
        id: string,
        publisherObject: any
    ): Promise<any> {
        const response = await api.put(`${this.apiUrl}alter/${id}`, publisherObject)

        return response;
    }

    async deletePublisher(id: string, user_id: string): Promise<any> {
        const response = await api.delete(`${this.apiUrl}delete?id=${id}&user_id=${user_id}`)

        return response;
    }

    async clearDeletedPublishers(): Promise<any> {
        const response = await api.get(`${this.apiUrl}clear_deleted`);

        return response;
    }
}