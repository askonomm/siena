<?php

namespace Askonomm\Siena;

use Symfony\Component\Yaml\Yaml;
use Ramsey\Uuid\Uuid;

/**
 * Siena is a flat-file data engine that uses the YAML format 
 * for storage.. It takes care of things such as creating, updating 
 * and querying for data.
 * 
 * @author Asko Nomm <asko@bien.ee>
 */
class Siena
{
    public function __construct(
        private readonly string $storeDir,
    ) {
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
     * Undocumented function
     *
     * @param string $path
     * @return array
     */
    private function getAll(string $path): array
    {
        $fullPath = $this->storeDir . '/' . $path . '/*.yaml';
        $items = [];

        foreach (glob($fullPath) as $item) {
            $items[] = static::get($item);
        }

        return $items;
    }

    public function get(string $path): ?array
    {
        $fullPath = $this->storeDir . '/' . $path . '.yaml';

        if (str_contains($path, $this->storeDir)) {
            $fullPath = $path;
        }

        if (file_exists($fullPath)) {
            return [
                ...Yaml::parseFile($fullPath),
                '_id' => $this->getIdFromPath($fullPath),
                '_path' => $fullPath,
            ];
        }

        return null;
    }

    public function find(string $directory): Search
    {
        return new Search($this->getAll($directory));
    }

    public function put(string $path, array $data): ?string
    {
        // Construct path
        $fullPath = $this->storeDir . '/' . $path . '.yaml';

        if (str_contains($path, $this->storeDir)) {
            $fullPath = $path;
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

    public function update(string $path, array $data): void
    {
        $item = $this->get($path);

        $this->put($path, [
            ...$item,
            ...$data,
        ]);
    }

    public function remove(string $path, array $rules = []): void
    {
        // If no rules are provided and the `$path` leads to an actual file, 
        // then let's straight up delete it from `$path`.
        if (empty($rules) && is_file($this->storeDir . '/' . $path . '.yaml')) {
            unlink($path);
        }

        // Otherwise, let's try to find the file according to `$rules`. 
        $item = $this->find($path)->where($rules)->first();

        if ($item) {
            unlink($item['_path']);
        }
    }
}
