
export interface EtsyPrice {
    amount: number;
    divisor: number;
    currency_code: string;
}

export interface EtsyImage {
    url_75x75?: string;
    url_170x135?: string;
    url_570xN?: string;
    url_fullxfull?: string;
}

export interface EtsyShop {
    shop_id: number;
    shop_name: string;
}

export interface EtsyListing {
    listing_id: number;
    title: string;
    description?: string;
    url: string;
    price?: EtsyPrice;
    num_favorers?: number;
    creation_tsz?: number;
    tags?: string[];
    images?: EtsyImage[];
    shop?: EtsyShop;
}

export async function getEtsyTop(limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('get_etsy_top', { limit, offset });
}

export async function getEtsyNew(limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('get_etsy_new', { limit, offset });
}

export async function searchEtsy(query: string, limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('search_etsy', { query, limit, offset });
}
