import { Author } from "../entities/author";


export interface iAuthorRepository {
    createAuthor(authorObject: any): Promise<any>;

    getAuthorById(id: string): Promise<Author>;

    getAuthorByName(name: string, skip?: number, page_size?: number): Promise<Author[]>;

    getAuthorsByBook(book_id: string, skip?: number, page_size?: number): Promise<Author[]>;

    getAuthorsByGender(gender_id: string, skip?: number, page_size?: number): Promise<Author[]>;

    getAuthorsByPublisher(publisher_id: string, skip?: number, page_size?: number): Promise<Author[]>;

    morePopularAuthor(skip?: number, page_size?: number): Promise<Author[]>;

    bestValuatedAuthor(skip?: number, page_size?: number): Promise<Author[]>;

    alterAuthor(id: string, authorObject: any): Promise<any>;

    deleteAuthor(id: string, user_id: string): Promise<any>;

    clearDeletedAuthors(): Promise<any>;
}