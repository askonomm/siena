<?php

namespace Askonomm\Siena;

/**
 * The QueryBuilder class takes in a `Store`, and then 
 * lets you construct a search query to find exactly 
 * the `StoreItem`'s you want from the `Store`.
 * 
 * @author Asko Nomm <asko@bien.ee>
 */
class QueryBuilder
{
    public function __construct(
        private Store $store = new Store([]),
    ) {
    }

    /**
     * Undocumented function
     *
     * @param array $conditions
     * @return self
     */
    public function where(array $conditions): self
    {
        $matchedItems = [];

        foreach ($this->store as $item) {
            $requirements = count($conditions);

            foreach ($conditions as $k => $v) {
                $keys = explode('|', $k);

                foreach ($keys as $key) {
                    if (isset($item->$key) && $item->$key === $v) {
                        $requirements--;

                        break;
                    }
                }
            }

            if ($requirements === 0) {
                $matchedItems[] = $item;
            }
        }

        $this->store = new Store($matchedItems);

        return $this;
    }

    public function orderAsc(string $key): self
    {
        $items = $this->store->toArray();

        usort($items, function ($a, $b) use ($key) {
            return $b[$key] <=> $a[$key];
        });

        $this->store = new Store($items);

        return $this;
    }

    public function orderDesc(string $key): self
    {
        $items = $this->store->toArray();

        usort($items, function ($a, $b) use ($key) {
            return $a[$key] <=> $b[$key];
        });

        $this->store = new Store($items);

        return $this;
    }

    public function limit(int $limit): self
    {
        $items = $this->store->toArray();
        $this->store = new Store(array_slice($items, 0, $limit));

        return $this;
    }

    public function offset(int $offset): self
    {
        $items = $this->store->toArray();
        $this->store = new Store(array_slice($items, $offset));

        return $this;
    }

    public function get(): Store
    {
        return $this->store;
    }

    public function first(): ?StoreItem
    {
        return $this->store->first();
    }

    public function last(): ?StoreItem
    {
        return $this->store->last();
    }
}
