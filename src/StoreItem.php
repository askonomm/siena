<?php

namespace Askonomm\Siena;

use Traversable;
use ArrayIterator;
use IteratorAggregate;

class StoreItem implements IteratorAggregate
{
    public function __construct(array $data)
    {
        foreach ($data as $k => $v) {
            $this->$k = $v;
        }
    }

    public function __get($name)
    {
        return $this->{$name};
    }

    public function getIterator(): Traversable
    {
        return new ArrayIterator($this);
    }
}
