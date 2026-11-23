class Dispatch {
  int route(int code, Color color) {
    switch (code) {
      case 1:
      case 2:
        prepare();
        break;
      case LIMIT - 1, 4:
        handle(code);
        return code;
      default:
        fallback();
    }
    switch (color.kind()) {
      case RED:
        stop();
        break;

      default:
    }
    switch (code) {
      case 3: {
        int n = code;
        handle(n);
      }
      case 4:
      case 5: {
        prepare();
      }
      case 6:
        {
          stop();
        }
        break;
      default: {
        fallback();
      }
    }
    return 0;
  }
}
