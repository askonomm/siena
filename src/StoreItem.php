<?php

namespace Askonomm\Siena;

class StoreItem
{
    public function __construct(array $kvs)
    {
        foreach ($kvs as $k => $v) {
            $this->$k = $v;
        }
    }

    public function get(string $key, mixed $default = null): ?mixed
    {
        if (isset($this->$key)) {
            return $this->$key;
        }

        return $default;
    }
}
