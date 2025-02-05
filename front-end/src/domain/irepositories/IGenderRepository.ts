import { Gender } from "../entities/Gender";

export interface IGenderRepository {
    getAllGenders(): Promise<Gender[]>;
    getGenderById(id: number): Promise<Gender>;
    saveGender(gender: Gender): Promise<void>;
}