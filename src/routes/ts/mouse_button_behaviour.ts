export interface MouseButtonBehaviour<Args extends unknown[], Returne>{
	down?: (...args: Args) => Returne;
	up  ?: (...args: Args) => Returne;
	move?: (...args: Args) => Returne;
	contextmenu?: (...args: Args) => Returne;
}
