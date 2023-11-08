-- Your SQL goes here
CREATE TABLE businesses_nodes (
 id serial primary key,
 business_id integer not null,
 node_id int not null,
 path text not null,
 host text not null,
 port int not null,
 expiry int not null, --duración de invoice
 cltv int not null, --hold invoice cltv delta (expiration time in blocks)
 max_paths int not null, -- maxima cantidad de saltos que queremos que el nodo de para intentar efectuar el pago
 pathfinding_timeout int not null, --cantidad de tiempo para tratar de encontrar una ruta
 max_fee decimal not null, -- la cantidad maxima de fee que esta mos dispuestos a pagar por el ruteo
 out text not null -- el ID del canal del peer por el cual queremos sacar el pago de nuestro nodo
);

alter table businesses_nodes
   add constraint fk_businesses_nodes_businesses foreign key (business_id)
      references businesses (id)
      on delete restrict on update restrict;
	  

create unique index idx_cn_businesses_nodes on businesses_nodes (
 business_id,  
 host
);