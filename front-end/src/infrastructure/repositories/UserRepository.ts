import { User } from "../../domain/entities/User";
import type { IUserRepository } from "../../domain/irepositories/IUserRepository";
import api from "../api";


export class UserRepository implements IUserRepository {
    private readonly apiUrl = "/users";

    async getAllUsers(): Promise<User[]> {
        const response = await api.get(this.apiUrl);
        return response.data.map((user: any) => {
            new User(
                user._id,
                user._name,
                user._nickname,
                user._email,
                user._password,
                user._birthData,
                user._registrationData,
                user._avatar
            )
        })
    }

    async getUserById(id: number): Promise<User> {
        const response = await api.get(`${this.apiUrl}/${id}`);
        return new User(
            response.data._id,
            response.data._name,
            response.data._nickname,
            response.data._email,
            response.data._password,
            response.data._birthData,
            response.data._registrationData,
            response.data._avatar
        )
    }

    async getUserByUsername(username: string): Promise<User[]> {
        const response = await api.get(`${this.apiUrl}?username=${username}`);
        return response.data.map((user: any) => {
            new User(
                user._id,
                user._name,
                user._nickname,
                user._email,
                user._password,
                user._birthData,
                user._registrationData,
                user._avatar
            )
        })
    }

    async getUserByNickname(nickname: string): Promise<User[]> {
        const response = await api.get(`${this.apiUrl}?nickname=${nickname}`);
        return response.data.map((user: any) => {
            new User(
                user._id,
                user._name,
                user._nickname,
                user._email,
                user._password,
                user._birthData,
                user._registrationData,
                user._avatar
            )
        })
    }

    async getUserByEmail(nickname: string): Promise<User[]> {
        const response = await api.get(`${this.apiUrl}?mail=${nickname}`);
        return response.data.map((user: any) => {
            new User(
                user._id,
                user._name,
                user._nickname,
                user._email,
                user._password,
                user._birthData,
                user._registrationData,
                user._avatar
            )
        })
    }

    async saveUser(user: User): Promise<void> {
        await api.post(this.apiUrl, user);
    }
}