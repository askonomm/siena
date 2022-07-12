<?php

namespace Askonomm\Siena;

use Symfony\Component\Yaml\Yaml;
use Ramsey\Uuid\Uuid;

/**
 * Siena is a flat-file data store engine that uses the YAML format 
 * for storage. It comes featured with a Query builer and takes care 
 * of things such as creating, updating, querying and deleting of data, 
 * which makes it easy to build software solutions using just a 
 * flat-file data store.
 * 
 * @author Asko Nomm <asko@bien.ee>
 */
class Siena
{
    public function __construct(
        private readonly string $storeDir,
    ) {
        if (!is_dir($storeDir)) {
            mkdir($storeDir);
        }
    }

    /**
     * Strips the .yaml file extension from `$input`.
     *
     * @param string $input
     * @return string
     */
    private function stripExt(string $input): string
    {
        return str_replace('.yaml', '', $input);
    }

    /**
     * Given a file `$path`, returns the name of it, without extension, 
     * which serves as the item' ID.
     *
     * @param string $path
     * @return string
     */
    private function getIdFromPath(string $path): string
    {
        $parts = explode('/', $path);

        return str_replace('.yaml', '', end($parts));
    }

    /**
     * Given a `$directory`, get a `Store` of items in it.
     *
     * @param string $directory
     * @return Store
     */
    private function getAll(string $directory): Store
    {
        $fullPath = $this->storeDir . '/' . $directory . '/*.yaml';
        $items = [];

        foreach (glob($fullPath) as $item) {
            $items[] = $this->get($item);
        }

        return new Store($items);
    }

    /**
     * Given a `$path`, get a `StoreItem`. 
     * 
     * Example usage:
     * ```php
     * $siena = new Siena();
     * $siena->get('posts/hello-world');
     * ```
     *
     * @param string $path
     * @return StoreItem|null
     */
    public function get(string $pathToFile): ?StoreItem
    {
        $fullPath = $this->storeDir . '/' . $this->stripExt($pathToFile) . '.yaml';

        if (str_contains($pathToFile, $this->storeDir)) {
            $fullPath = $pathToFile;
        }

        if (file_exists($fullPath)) {
            return new StoreItem([
                ...Yaml::parseFile($fullPath),
                '_id' => $this->getIdFromPath($fullPath),
                '_path' => $fullPath,
            ]);
        }

        return null;
    }

    /**
     * Gets all items in `$directory` and passes it to the 
     * Query class, and returns its instance for manipulation.
     *
     * @param string $directory
     * @return QueryBuilder
     */
    public function find(string $directory): QueryBuilder
    {
        return new QueryBuilder($this->getAll($directory));
    }

    /**
     * Creates an item in `$pathToFile` and adds given `$data` in
     * it. If a special `:id` token is used within `$pathToFile`, 
     * will create a UUID V4 ID in that place, and return that ID, 
     * otherwise returns null.
     * 
     * Example use:
     * 
     * ```php
     * $siena = new Siena();
     * $siena->put('posts/hello-world', ['title' => 'Hello, World!']);
     * ```
     * 
     * Or if you want to generate a unique ID:
     * 
     * ```php
     * $siena = new Siena();
     * $id = $siena->put('posts/:id', ['title' => 'Hello, World!']);
     * ```
     * 
     * And now `$id` would be something like `ae29f58e-b253-49f6-a557-7f0fa315c9d9`.
     *
     * @param string $pathToFile
     * @param array $data
     * @return string|null
     */
    public function put(string $pathToFile, array $data): ?string
    {
        // Construct path
        $fullPath = $this->storeDir . '/' . $this->stripExt($pathToFile) . '.yaml';

        if (str_contains($pathToFile, $this->storeDir)) {
            $fullPath = $pathToFile;
        }

        // Generate ID
        $generatedId = Uuid::uuid4();

        // If the directory does not exist, create it.
        $dirname = dirname(str_replace(':id', $generatedId, $fullPath));

        if (!is_dir($dirname)) {
            mkdir($dirname, 0777, true);
        }

        // Unset transient data
        unset($data['_id']);
        unset($data['_path']);

        // Store data
        file_put_contents(str_replace(':id', $generatedId, $fullPath), Yaml::dump($data));

        if (str_contains($fullPath, ':id')) {
            return $generatedId;
        }

        return null;
    }

    /**
     * Updates an item at `$pathToFile` with `$data`. If data with 
     * existing keys exist, they will be overwritten, otherwise added.
     *
     * @param string $pathToFile
     * @param array $data
     * @return void
     */
    public function update(string $pathToFile, array $data): void
    {
        $item = $this->get($pathToFile);

        if ($item) {
            $this->put($pathToFile, [
                ...$item,
                ...$data,
            ])
        } else {
            $this->put($pathToFile, $data);
        }
    }

    /**
     * Deletes the first file in `$directory` that the `QueryBuilder` 
     * matches with given `$where` conditions.
     *
     * @param string $pathToFile
     * @param array $where
     * @return void
     */
    public function remove(string $directory, array $where = []): void
    {
        $item = $this->find($directory)->where($where)->first();

        if ($item) {
            unlink($item['_path']);
        }
    }

    /**
     * Removes all files in `$directory` that the `QueryBuilder` 
     * matches with given `$where` conditions.
     *
     * @param string $pathToFile
     * @param array $rules
     * @return void
     */
    public function removeAll(string $directory, array $where = []): void
    {
        $items = $this->find($directory)->where($where)->get();

        foreach ($items as $item) {
            unlink($item['_path']);
        }
    }
}
