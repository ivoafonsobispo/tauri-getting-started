const { invoke } = window.__TAURI__.tauri;

let pokemonNameInputEl;
let pokemonMsgEl;
let pokemonImgEl;
let pokemonNameEl;
let pokemonWeightEl;
let pokemonHeightEl;
let pokemonIdEl;

async function getPokemon() {
  
  let pokemon = await invoke("get_pokemon", { name: pokemonNameInputEl.value });

  // Parsing response
  let pokemonJson = '{'+pokemon.replace('Pokemon {', '').replace('}', '').replace('id', '"id"').replace('name', '"name"').replace('weight', '"weight"').replace('height', '"height"').trim()+'}';

  // 3. Parse the remaining string as JSON

  // 4. Update the field's values 
  
  // 4.1. Update Message ("You've chosen Pikachu!")

  // 4.2. Update Pokemon's Pokédex Number 

  // 4.3. Update Pokemon's Name

  // 4.4. Update Pokemon's Weight

  // 4.5. Update Pokemon's Height

  // 4.6. Update Pokemon's Image (for the image source use: https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/<pokemon_id>.png)


}

window.addEventListener("DOMContentLoaded", () => {

  pokemonNameInputEl = document.querySelector("#pokemon-name-input");
  pokemonMsgEl = document.querySelector("#pokemon-msg");
  pokemonImgEl = document.querySelector("#pokemon-img");
  pokemonNameEl = document.querySelector("#pokemon-name");
  pokemonWeightEl = document.querySelector("#pokemon-weight");
  pokemonHeightEl = document.querySelector("#pokemon-height");
  pokemonIdEl = document.querySelector("#pokemon-id");

  document.querySelector("#pokemon-form").addEventListener("submit", (e) => {
    e.preventDefault();
    // 5. Add the function to the form's button
  });
});
