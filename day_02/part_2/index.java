import java.util.Scanner;

class Moves {
  static String[] moves = { "R", "P", "S" };

  static int indexOf(String move) {
    for (int i = 0; i < 3; i++) {
      if (Moves.moves[i].equals(move)) {
        return i;
      }
    }
    System.out.println(move);
    throw new Error("Incorrect move");
  }

  static String play(String oponent, String player) {
    int direction = 0;
    if ("X".equals(player)) {
      direction = -1;
    } else if ("Z".equals(player)) {
      direction = 1;
    }
    return Moves.moves[(Moves.indexOf(oponent) + direction + 3) % 3];
  }

}

class Source {
  public static Scanner scan = new Scanner(System.in);

  public static void main(String[] args) {
    long points = 0;
    System.out.println("HELLO");
    while (scan.hasNextLine()) {
      String[] moves = scan.nextLine().split(" ");
      System.out.println(inputToMove(moves[0]) + inputToMove(moves[1]));
      if ("Y".equals(moves[1])) {
        points += 3;
      } else if ("Z".equals(moves[1])) {
        points += 6;
      }
      points += movePoints(Moves.play(inputToMove(moves[0]), moves[1]));
    }

    System.out.println(points);

  }

  public static String inputToMove(String input) {
    if ("A".equals(input)) {
      return "R";
    } else if ("B".equals(input)) {
      return "P";
    } else if ("C".equals(input)) {
      return "S";
    } else if ("X".equals(input)) {
      return "R";
    } else if ("Y".equals(input)) {
      return "P";
    } else if ("Z".equals(input)) {
      return "S";
    } else {
      throw new Error("Invalid input");
    }
  }

  public static long movePoints(String move) {
    if ("R".equals(move)) {
      return 1;
    }
    if ("P".equals(move)) {
      return 2;
    }
    return 3;
  }

}