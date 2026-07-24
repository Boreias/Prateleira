export function genderObject(genderName: string, user_id: string, gender_id?: string, book_ids?: string[]) {
    return {
        "id": gender_id,
        "name": genderName,
        "user_id": user_id,
        "book_ids": book_ids
    }
}

export function publisherObject(
    name: string,
    user_id: string,
    publisher_id?: string,
    site?: string,
    email?: string,
    avatar?: string
) {
    return {
        "id": publisher_id,
        "name": name,
        "site": site,
        "email": email,
        "avatar": avatar,
        "user_id": user_id
    }
}

export function authorObject(
    name: string,
    user_id: string,
    author_id?: string,
    avatar?: string,
    book_ids?: string[]
) {
    return {
        "id": author_id,
        "name": name,
        "avatar": avatar,
        "user_id": user_id,
        "book_ids": book_ids
    }
}

export function bookObject(
    title: string,
    user_id: string,
    authors: string[],
    publisher: string,
    isbn: string,
    genders: string[],
    subtitle?: string,
    series_collection?: number,
    volume?: number,
    edition?: number,
    publication_year?: number,
    pages?: number,
    language?: string,
    synopsis?: string,
    cover?: string
) {
    return {
        "title": title,
        "user_id": user_id,
        "authors": authors,
        "publisher": publisher,
        "isbn": isbn,
        "genders": genders,
        "subtitle": subtitle,
        "series_collection": series_collection,
        "volume": volume,
        "edition": edition,
        "publication_year": publication_year,
        "pages": pages,
        "language": language,
        "synopsis": synopsis,
        "cover": cover
    }
}