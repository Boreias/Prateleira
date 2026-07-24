import { Gender } from "../entities/gender";


export interface iGenderRepository {
    createGender(genderObject: any): Promise<any>;

    getGenderById(id: string): Promise<Gender>;

    getGenderByName(name: string, skip: number, page_size: number): Promise<Gender[]>;

    getGendersByBook(book_id: string, skip: number, page_size: number): Promise<Gender[]>;

    getGendersByAuthor(author_id: string, skip: number, page_size: number): Promise<Gender[]>;

    getGendersByPublisher(publisher_id: string, skip: number, page_size: number): Promise<Gender[]>;

    morePopularGender(skip: number, page_size: number): Promise<Gender[]>;

    bestValuatedGender(skip: number, page_size: number): Promise<Gender[]>;

    alterGender(id: string, genderObject: any): Promise<any>;

    deleteGender(id: string, user_id: string): Promise<any>;

    clearDeletedGenders(): Promise<any>;
}