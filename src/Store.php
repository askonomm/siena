<?php

namespace Askonomm\Siena;

use ArrayIterator;
use IteratorAggregate;
use Traversable;

class Store implements IteratorAggregate
{
    public function __construct(
        private array $items,
    ) {
    }

    public function count(): int
    {
        return count($this->items);
    }

    public function first(): ?StoreItem
    {
        if ($this->count() > 0) {
            return $this->items[0];
        }

        return null;
    }

    public function last(): ?StoreItem
    {
        if ($this->count() > 0) {
            $items = $this->items;

            return end($items);
        }

        return null;
    }

    public function toArray(): array
    {
        return $this->items;
    }

    public function getIterator(): Traversable
    {
        return new ArrayIterator($this->items);
    }
}
