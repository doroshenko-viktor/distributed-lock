# Distributed Lock Server

## Message Format

```text
<OPERATION>\n
<LOCK_KEY>\n
\n
<META>
```

`OPERATION` may be one of:

- `LOCK`
- `RELEASE`
- ...

`LOCK_KEY` is a string key, which uniquely identifies lock.

`META` - contains additional information for request, such as:

- `TYPE` - type of lock created:

  - Release locked clients by their stack position. Those, which requested earlier will be released earlier;
  - Release firstly those, which were locked last;
  - Release all clients on unlock;
