import { Publisher } from "../entities/publisher";


export interface iPublisherRepository {
    createPublisher(publisherObject: any): Promise<any>;

    getPublisherById(id: string): Promise<Publisher>;

    getPublisherByName(name: string, skip: number, page_size: number): Promise<Publisher[]>;

    getPublisherByBook(book_id: string, skip: number, page_size: number): Promise<Publisher>;

    getPublishersByAuthor(author_id: string, skip: number, page_size: number): Promise<Publisher[]>;

    getPublishersByGender(gender_id: string, skip: number, page_size: number): Promise<Publisher[]>;

    morePopularPublishers(skip: number, page_size: number): Promise<Publisher[]>;

    bestValuatedPublishers(skip: number, page_size: number): Promise<Publisher[]>;

    alterPublisher(id: string, publisherObject: any): Promise<any>;

    deletePublisher(id: string, user_id: string): Promise<any>;

    clearDeletedPublishers(): Promise<any>;
}