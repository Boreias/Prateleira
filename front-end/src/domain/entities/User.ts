export class User {
    constructor(
        private readonly _id: number,
        private _name: string,
        private _nickname: string,
        private _mail: string,
        private _password: string,
        private _birthData: Date,
        private _registrationData: Date
    ){
        this._id = _id;
        this._name = _name;
        this._nickname = _nickname;
        this._mail = _mail;
        this._password = _password;
        this._birthData = _birthData;
        this._registrationData = _registrationData;
    }

    get id(){
        return this._id;
    }

    get name(){
        return this._name;
    }

    get nickname(){
        return this._nickname;
    }

    get mail(){
        return this._mail;
    }

    get password(){
        return this._password;
    }

    get birthData(){
        return this._birthData;
    }

    get registrationData(){
        return this._registrationData;
    }

    setName(name: string){
        this._name = name;
    }

    setNickname(nickname: string){
        this._nickname = nickname;
    }

    setMail(mail: string){
        this._mail = mail;
    }

    setPassword(password: string){
        this._password = password;
    }

    setBirthData(birthData: Date){
        this._birthData = birthData;
    }

    setRegistrationData(registrationData: Date){
        this._registrationData = registrationData;
    }
}