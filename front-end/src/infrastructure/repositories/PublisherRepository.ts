import { Publisher } from "../../domain/entities/Publisher";
import type { IPublisherRepository } from "../../domain/irepositories/IPublisherRepository";
import api from "../api";



export class PublisherRepository implements IPublisherRepository {
    private readonly apiUrl = "/publishers";

    async getAllPublishers(): Promise<Publisher[]> {
        const response = await api.get(this.apiUrl);
        return response.data.map((publisher: any) => {
            new Publisher(
                publisher._id,
                publisher._name,
                publisher._site,
                publisher._email,
                publisher._avatar,
                publisher._books
            )
        })
    }

    async getPublisherById(id: number): Promise<Publisher> {
        const response = await api.get(`${this.apiUrl}/${id}`);
        return new Publisher(
            response.data._id,
            response.data._name,
            response.data._site,
            response.data._email,
            response.data._avatar,
            response.data._books
        )
    }

    async getPublisherByName(name: string): Promise<Publisher> {
        const response = await api.get(`${this.apiUrl}?name=${name}`);
        return new Publisher(
            response.data._id,
            response.data._name,
            response.data._site,
            response.data._email,
            response.data._avatar,
            response.data._books
        )
    }

    async savePublisher(publisher: any): Promise<void> {
        await api.post(this.apiUrl, publisher);
    }
}