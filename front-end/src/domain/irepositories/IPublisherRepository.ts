import { Publisher } from "../entities/Publisher";

export interface IPublisherRepository {
    getAllPublishers(): Promise<Publisher[]>;
    getPublisherById(id: number): Promise<Publisher>;
    getPublisherByName(name: string): Promise<Publisher>;
    savePublisher(publisher: Publisher): Promise<void>;
}