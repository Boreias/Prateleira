import { User } from "../entities/User";


export interface IUserRepository {
    getAllUsers(): Promise<User[]>;
    getUserById(id: number): Promise<User>;
    getUserByUsername(username: string): Promise<User[]>;
    getUserByNickname(nickname: string): Promise<User[]>;
    saveUser(user: User): Promise<void>;
}