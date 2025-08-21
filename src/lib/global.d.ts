

interface Dishes {
    id: number;
    name_en: string;
    name_cn: string;
    menu_type: string;
    price: number;
    is_set_meal: boolean;
    is_attached: boolean;
    is_selectable: boolean;
    notes: string | null;
    img: string;
}

interface Types {
    id: number;
    name_en: string;
    name_cn: string;
    img: string;
}