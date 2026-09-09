# Decision Recording Policy

Create a durable decision record only when the rationale must survive the current task and materially constrains future work.

Good candidates:

- selected application/UI/runtime stack;
- irreversible or expensive architecture boundary;
- public data/privacy contract;
- deliberate compatibility break;
- long-lived external service/provider choice.

Do not create decision records for routine implementation details, obvious fixes, temporary experiments, or facts already owned by foundation documents.

Each decision should state:

```text
Decision
Context
Options considered
Why selected
Consequences / constraints
Supersedes (if any)
```
