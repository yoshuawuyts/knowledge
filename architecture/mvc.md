# mvc
Model-View-Controller is perhaps the best known architecture paradigm. It
splits data representation into a domain Model, sets up layers of glue code
around the Model in the Controller, and has a public interface in the form of a
view (either graphical or API). It's been around for a long time, and offers
a great balance between separation of concerns and few layers.

## MVC for servers
The interface for servers is the API. Data from the user comes in through the
API which checks if requests are valid, and then hands off control to a
controller. Controllers glue together Models to provide complex behavior from
single actions. E.g. when deleting a user, all users connected to that user
might need to update their relationship (e.g. unfriend or similar).

Having 3 layers of separation untangles internals from interfaces and provides
a strong focus on reusability. Though it is possible to add more layers to
further decouple elements, adding it often a signal that the scope of the
application has grown too large and should be split into a separate repository.

__conceptual separation__
```txt
      (user)
-----------------
   API gateway      Validate request and call the right controller
-----------------
   Controllers      Call the right Models to get combinations of behavior
-----------------
      Model         Single actions that operate on types of data
-----------------
 (ORM / adapter)
```

__internal module separation__
```txt
     app-main       Server initialization logic
-----------------
     app-api        API gateway
-----------------
 app-controllers    Controllers
-----------------
   app-models       Models
```

__application layers__
```txt
      HTTP handler       TCP / HTTP handler
----------------------
 Dependency injection    Transparently pass references and methods down
----------------------
      Middleware         Mostly reserved for logging, often wrongly extended
----------------------
       Routing           Determine which path to take
----------------------
```
