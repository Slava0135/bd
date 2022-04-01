CREATE TABLE IF NOT EXISTS public.orders (
    id serial PRIMARY KEY,
    creation_time timestamp NOT NULL,
    email varchar(255) NOT NULL,
    phone_number varchar(50)
);
CREATE TABLE IF NOT EXISTS public.passengers (
    id serial PRIMARY KEY,
    first_name varchar(50) NOT NULL CHECK (first_name != ''),
    last_name varchar(50) NOT NULL CHECK (last_name != ''),
    gender char NOT NULL,
    document varchar(255) NOT NULL UNIQUE CHECK (document != ''),
    phone_number varchar(50)
);
CREATE TABLE IF NOT EXISTS public.routes (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE CHECK (name != ''),
    first_station varchar(50) NOT NULL CHECK (first_station != ''),
    last_station varchar(50) NOT NULL CHECK (last_station != ''),
    CONSTRAINT valid_route CHECK (first_station != last_station)
);
CREATE TABLE IF NOT EXISTS public.route_sections (
    id serial PRIMARY KEY,
    route_id integer REFERENCES routes NOT NULL,
    departure_station varchar(50) NOT NULL CHECK (departure_station != ''),
    departure_time interval NOT NULL,
    destination_station varchar(50) NOT NULL CHECK (destination_station != ''),
    destination_time interval NOT NULL,
    cost real NOT NULL CHECK (cost > 0),
    CONSTRAINT valid_section CHECK (
        departure_station != destination_station
        AND departure_time < destination_time
    )
);
CREATE TABLE IF NOT EXISTS public.wagon_classes (
    id serial PRIMARY KEY,
    name varchar(50) NOT NULL UNIQUE CHECK (name != ''),
    capacity integer NOT NULL CHECK (capacity > 0),
    cost_multiplier real NOT NULL CHECK (cost_multiplier > 0)
);
CREATE TABLE IF NOT EXISTS public.trains (
    id serial PRIMARY KEY,
    route_id integer REFERENCES routes NOT NULL,
    length integer NOT NULL CHECK (length > 0),
    departure_date date NOT NULL
);
CREATE TABLE IF NOT EXISTS public.train_wagons (
    train_id integer REFERENCES trains,
    wagon_id integer REFERENCES wagon_classes,
    position_in_train integer CHECK (position_in_train > 0),
    PRIMARY KEY (train_id, wagon_id, position_in_train)
);
CREATE TABLE IF NOT EXISTS public.tickets (
    id serial PRIMARY KEY,
    train_id integer REFERENCES trains NOT NULL,
    order_id integer REFERENCES orders NOT NULL,
    passenger_id integer REFERENCES passengers NOT NULL,
    departure_station varchar(50) NOT NULL CHECK (departure_station != ''),
    departure_time timestamp NOT NULL,
    destination_station varchar(50) NOT NULL CHECK (destination_station != ''),
    destination_time timestamp NOT NULL,
    wagon_number integer NOT NULL CHECK (wagon_number > 0),
    seat integer NOT NULL CHECK (seat > 0),
    price_paid decimal NOT NULL CHECK (price_paid > 0)
);