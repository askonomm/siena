<?php

namespace Askonomm\Siena;

/**
 * Undocumented class
 * 
 * @author Asko Nomm <asko@bien.ee>
 */
class Query
{
    public function __construct(
        private array $items = [],
    ) {
    }

    public function where(array $conditions): self
    {
        $matchedItems = [];

        foreach ($this->items as $item) {
            $requirements = count($conditions);

            foreach ($conditions as $k => $v) {
                $keys = explode('|', $k);

                foreach ($keys as $key) {
                    if (isset($item[$key]) && $item[$key] === $v) {
                        $requirements--;

                        break;
                    }
                }
            }

            if ($requirements === 0) {
                $matchedItems[] = $item;
            }
        }

        $this->items = $matchedItems;

        return $this;
    }

    public function orderAsc(string $key): self
    {
        usort($this->items, function ($a, $b) use ($key) {
            return $b[$key] <=> $a[$key];
        });

        return $this;
    }

    public function orderDesc(string $key): self
    {
        usort($this->items, function ($a, $b) use ($key) {
            return $a[$key] <=> $b[$key];
        });

        return $this;
    }

    public function get(): array
    {
        return $this->items;
    }

    public function first(): ?array
    {
        if (isset($this->items) && count($this->items) > 0) {
            return $this->items[0];
        }

        return null;
    }

    public function last(): ?array
    {
        if (isset($this->items) && count($this->items) > 0) {
            $items = $this->items;

            return end($items);
        }

        return null;
    }
}
