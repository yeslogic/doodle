export class Env {
  extend(entry) {
    return new Extend(this, entry);
  }
}

export class Extend extends Env {
  constructor(env, entry) {
    this.env = env;
    this.entry = entry;
  }

  lookup(index) {
    if (index === 0) {
      return this.entry;
    } else if (index > 0) {
      return this.env.lookup(index - 1);
    } else {
      throw 'index out of range';
    }
  }
}

export class Empty extends Env {
  constructor() { }

  lookup(_) {
    return null;
  }
}

export function evaluate(env, expr) {
  switch (expr.tag) {
    case 'Var':
      return env.lookup(expr.data);
    case 'Bool':
    case 'U8':
    case 'U16':
    case 'U32':
      return expr;
    case 'Tuple':
      return {
        tag: 'Tuple',
        data: expr.data.map(item => evaluate(env, item))
      };
    case 'TupleProj': {
      const [head, index] = expr.data;
      const item = evaluateTuple(env, head)[index];
      if (item) {
        return item;
      } else {
        throw `item ${index} not found in tuple`;
      }
    }
    case 'Record':
      return {
        tag: 'Record',
        data: expr.data.map(([label, fieldExpr]) =>
          [label, evaluate(env, fieldExpr)]
        )
      };
    case 'RecordProj': {
      const [head, label] = expr.data;
      for (const [fieldLabel, value] in evaluateRecord(env, head)) {
        if (fieldLabel === label) {
          return value;
        }
      }
      throw `field ${label} not found in record`;
    }
    case 'Variant': {
      const [label, variantExpr] = expr.data;
      return {
        tag: 'Variant',
        data: [label, evaluate(env, variantExpr)]
      };
    }
    case 'Seq':
      return {
        tag: 'Seq',
        data: value.data.map(item => evaluate(env, item))
      };
    case 'Match': {
      const [head, branches] = expr.data;
      const headValue = evaluate(env, head);
      for (const [pattern, branchExpr] in branches) {
        const extendedEnv = matches(env, pattern, headValue);
        if (extendedEnv) {
          return evaluate(extendedEnv, branchExpr)
        }
      }
      throw 'non-exhaustive patterns';
    }
    case 'BitAnd':
    case 'BitOr':
    case 'Eq':
    case 'Ne':
    case 'Rem':
    case 'Shl':
    case 'Add':
    case 'Sub':
    case 'U16Be':
    case 'U16Le':
    case 'U32Be':
    case 'U32Le':
    case 'Stream':
      throw 'not yet implemented';
    default:
      throw `unexpected tag ${expr.tag}`;
  }
}

function evaluateRecord(env, expr) {
  const value = evaluate(env, expr);
  switch (value.tag) {
    case 'Record':
      return value.data;
    default:
      throw 'value is not a record';
  }
}

function evaluateTuple(env, expr) {
  const value = evaluate(env, expr);
  switch (value.tag) {
    case 'Tuple':
      return value.data;
    default:
      throw 'value is not a tuple';
  }
}

function matches(env, pattern, value) {
  switch (pattern.tag) {
    case 'Binding':
      return env.extend(value);
    case 'Wildcard':
      return env;
    default:
      if (value.tag !== pattern.tag) {
        throw 'mismatched tags';
      }
      switch (pattern.tag) {
        case 'Bool':
        case 'U8':
        case 'U16':
        case 'U32':
          return pattern.data === value.data ? env : null;
        case 'Tuple':
        case 'Seq': {
          if (pattern.data.length !== value.data.length) {
            return null;
          }
          let extendedEnv = env;
          // FIXME: reverse order?
          for (let i = 0; i < pattern.data.length; i++) {
            extendedEnv = matches(extendedEnv, pattern.data[i], value.data[i]);
            if (!extendedEnv) {
              return null;
            }
          }
          return extendedEnv;
        }
        case 'Variant': {
          const [label0, p] = pattern.data;
          const [label1, v] = value.data;
          return label0 === label1 ? matches(env, p, v) : null;
        }
        default:
          throw `unexpected tag ${pattern.tag}`;
      }
  }
}
