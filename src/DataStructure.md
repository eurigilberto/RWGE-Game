# Data structure

## Collections

### Always present
- Entity collection
- Public data collection
	* Render Texture
	* Material
	* Transform

### Extra collections
- GUI collection
	- TODO
- Deferred Render System
	- When rendering an entity, the entity ***Must*** have a slotkey of material, and ***Should*** have a slotkey of transform. In this case it would be a single one per `Render Entity` 